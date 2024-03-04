use {
  super::*,
  bitcoin::{
    locktime::absolute::LockTime,
    policy::MAX_STANDARD_TX_WEIGHT,
    Witness,
  },
  bitcoincore_rpc::RawTx,
  std::{
    collections::BTreeSet,
    fs::File,
    io::{BufRead, BufReader},
  },
};

#[derive(Debug, Parser, Clone)]
pub(crate) struct SendMany {
  #[arg(
    long,
    help = "Consider spending outpoint <UTXO>, even if it is unconfirmed or contains inscriptions"
  )]
  utxo: Vec<OutPoint>,
  #[arg(long, help = "Use fee rate of <FEE_RATE> sats/vB")]
  fee_rate: FeeRate,
  #[arg(long, help = "Location of a CSV file containing `inscriptionid`,`destination` pairs.")]
  pub(crate) csv: PathBuf,
  #[arg(long, help = "Broadcast the transaction; the default is to output the raw tranasction hex so you can check it before broadcasting.")]
  pub(crate) broadcast: bool,
  #[arg(long, help = "Do not check that the transaction is equal to or below the MAX_STANDARD_TX_WEIGHT of 400,000 weight units. Transactions over this limit are currently nonstandard and will not be relayed by bitcoind in its default configuration. Do not use this flag unless you understand the implications."
  )]
  pub(crate) no_limit: bool,
  #[arg(long, help = "By default it is an error to list only some of the inscriptions in an output. This flag allows you to not care about the inscriptions you don't list in the CVS file.")]
  pub(crate) ignore_unlisted: bool,
  #[arg(long, help = "The smallest amount to use for each inscription output.")]
  pub(crate) min_postage: Option<Amount>,
  #[arg(long, help = "The largest amount to use for each inscription output.")]
  pub(crate) max_postage: Option<Amount>,
  #[arg(long, help = "The address to send cardinal outputs to.")]
  pub(crate) change: Option<Address<NetworkUnchecked>>,
  #[arg(long, help = "Which cardinal to use to pay the fees.")]
  pub(crate) cardinal: Option<OutPoint>,
}

#[derive(Serialize, Deserialize)]
pub struct Output {
  pub tx: String,
}

impl SendMany {
  const SCHNORR_SIGNATURE_SIZE: usize = 64;

  pub(crate) fn run(self, wallet: String, options: Options) -> SubcommandResult {
    let file = File::open(&self.csv)?;
    let reader = BufReader::new(file);
    let mut line_number = 1;
    let mut requested = BTreeMap::new();

    let chain = options.chain();

    if self.min_postage.is_some() && self.max_postage.is_some() && self.min_postage.unwrap() > self.max_postage.unwrap() {
        bail!("--min-postage {} sats is bigger than --max-postage {} sats", self.min_postage.unwrap().to_sat(), self.max_postage.unwrap().to_sat());
    }

    for line in reader.lines() {
      let line = line?;
      let mut line = line.trim_start_matches('\u{feff}').split(',');

      let inscriptionid = line.next().ok_or_else(|| {
        anyhow!("CSV file '{}' is not formatted correctly - no inscriptionid on line {line_number}", self.csv.display())
      })?;

      let inscriptionid = match InscriptionId::from_str(inscriptionid) {
        Err(e) => bail!("bad inscriptionid on line {line_number}: {}", e),
        Ok(ok) => ok,
      };

      let destination = line.next().ok_or_else(|| {
        anyhow!("CSV file '{}' is not formatted correctly - no comma on line {line_number}", self.csv.display())
      })?;

      let destination = match match Address::from_str(destination) {
        Err(e) => bail!("bad address on line {line_number}: {}", e),
        Ok(ok) => ok,
      }.require_network(chain.network()) {
        Err(e) => bail!("bad network for address on line {line_number}: {}", e),
        Ok(ok) => ok,
      };

      if requested.contains_key(&inscriptionid) {
        bail!("duplicate entry for {} on line {}", inscriptionid.to_string(), line_number);
      }

      requested.insert(inscriptionid, destination);
      line_number += 1;
    }

    let index = Index::open(&options)?;
    index.update()?;

    let client = bitcoin_rpc_client_for_wallet_command(wallet, &options)?;
    let mut unspent_outputs = get_unspent_outputs(&client, &index)?;
    let locked_outputs = get_locked_outputs(&client)?;

    for outpoint in &self.utxo {
      unspent_outputs.insert(
        *outpoint,
        Amount::from_sat(
          client.get_raw_transaction(&outpoint.txid, None)?.output[outpoint.vout as usize].value,
        ),
      );
    }

    // we get a vector of (SatPoint, InscriptionId), and turn it into a map <InscriptionId> -> <SatPoint>
    let mut inscriptions = BTreeMap::new();
    for (satpoint, inscriptionid) in index.get_inscriptions_vector(&unspent_outputs)? {
      inscriptions.insert(inscriptionid, satpoint);
    }

    let mut inputs = Vec::new();
    let mut outputs = Vec::new();

    let mut requested_satpoints: BTreeMap<SatPoint, (InscriptionId, Address)> = BTreeMap::new();

    // this loop checks that we own all the listed inscriptions, and that we aren't listing the same sat more than once
    for (inscriptionid, address) in &requested {
      if !inscriptions.contains_key(&inscriptionid) {
        bail!("inscriptionid {} isn't in the wallet", inscriptionid.to_string());
      }

      let satpoint = inscriptions[&inscriptionid];
      if requested_satpoints.contains_key(&satpoint) {
        bail!("inscriptionid {} is on the same sat as {}, and both appear in the CSV file", inscriptionid.to_string(), requested_satpoints[&satpoint].0);
      }
      requested_satpoints.insert(satpoint, (inscriptionid.clone(), address.clone()));
    }

    let change_dust_limit = Self::get_change_pubkey(&client, chain, self.change.clone())?.dust_value().to_sat();

    let mut cardinal_value = 0;
    // this loop handles the inscriptions in order of offset in each utxo
    while !requested.is_empty() {
      let mut inscriptions_on_outpoint;
      let mut inscriptions_to_send = Vec::new();
      // pick the first remaining inscriptionid from the list
      for (inscriptionid, _address) in &requested {
        // look up which utxo it's in
        let outpoint = inscriptions[inscriptionid].outpoint;
        // get a list of the inscriptions in that utxo
        inscriptions_on_outpoint = index.get_inscriptions_on_output_with_satpoints(outpoint)?;
        // sort it by offset
        inscriptions_on_outpoint.sort_by_key(|(s, _)| s.offset);
        // make sure that they are all in the csv file, unless --ignore-unlisted is in effect
        for (satpoint, outpoint_inscriptionid) in &inscriptions_on_outpoint {
          if self.ignore_unlisted {
            if requested_satpoints.contains_key(&satpoint) {
              inscriptions_to_send.push((satpoint, outpoint_inscriptionid));
            }
          } else {
            if !requested_satpoints.contains_key(&satpoint) {
              bail!("inscriptionid {} is in the same output as {} but wasn't in the CSV file", outpoint_inscriptionid.to_string(), inscriptionid.to_string());
            }
            inscriptions_to_send.push((satpoint, outpoint_inscriptionid));
          }
        }
        break;
      }

      // create an input for the first inscription of each utxo
      let (first_satpoint, _first_inscription) = inscriptions_to_send[0];
      let first_offset = first_satpoint.offset;
      let first_outpoint = first_satpoint.outpoint;
      let utxo_value = unspent_outputs[&first_outpoint].to_sat();
      if first_offset != 0 {
        cardinal_value += first_offset
      }
      inputs.push(first_outpoint);

      // filter out the inscriptions that aren't in our list, but are still to be sent - these are inscriptions that are on the same sat as the ones we listed
      // we want to remove just the ones where the satpoint is requested but that particular inscriptionid isn't
      // ie. keep the ones where the satpoint isn't requested or the inscriptionid is
      inscriptions_to_send = inscriptions_to_send.into_iter().filter(
        |(satpoint, inscriptionid)| !requested_satpoints.contains_key(&satpoint) || requested.contains_key(&inscriptionid)
      ).collect();

      // create an output for each inscription in this utxo
      for (i, (satpoint, inscriptionid)) in inscriptions_to_send.iter().enumerate() {
        if cardinal_value != 0 {
          outputs.push(TxOut{
            script_pubkey: Self::get_change_pubkey(&client, chain, self.change.clone())?,
            value: cardinal_value
          });
          cardinal_value = 0;
        }

        let destination = &requested_satpoints[&satpoint].1;
        let offset = satpoint.offset;
        let mut value = if i == inscriptions_to_send.len() - 1 { // if this is the last inscription in the output, use all the remaining sats
          utxo_value - offset
        } else { // else use the sats up to the next inscription
          inscriptions_to_send[i + 1].0.offset - offset
        };

        let script_pubkey = destination.script_pubkey();
        let dust_limit = script_pubkey.dust_value().to_sat();

        if let Some(min_postage) = self.min_postage {
          if value < min_postage.to_sat() {
            bail!("inscription {} at {} is only followed by {} sats, less than the specified --min-postage of {} sats",
                  inscriptionid, satpoint.to_string(), value, min_postage.to_sat());
          }
        }

        if let Some(max_postage) = self.max_postage {
          if value > max_postage.to_sat() {
            if value - max_postage.to_sat() >= change_dust_limit { // if using the max-postage size would leave a big enough change, do that
              cardinal_value = value - max_postage.to_sat();
              value -= cardinal_value;
            } else { // otherwise leave a big enough change
              cardinal_value = change_dust_limit;
              value -= cardinal_value;

              if let Some(min_postage) = self.min_postage {
                if value < min_postage.to_sat() {
                  bail!("trimming inscription {} at {} output of size {} sats so it doesn't exceed --max-postage {} sats leaves it smaller than --min-postage of {} sats",
                        inscriptionid, satpoint.to_string(), value, min_postage.to_sat(), max_postage.to_sat());
                }
              }
            }
          }
        }
        if value < dust_limit {
          bail!("inscription {} at {} would only have size {} sats, less than dust limit {} for address {}",
                inscriptionid, satpoint.to_string(), value, dust_limit, destination);
        }
        outputs.push(TxOut{script_pubkey, value});

        // remove each inscription in this utxo from the list
        requested.remove(&inscriptionid);
      }
    }

    let script_pubkey = Self::get_change_pubkey(&client, chain, self.change.clone())?;
    let value = 0; // we don't know how much change to take until we know the fee, which means knowing the tx vsize
    outputs.push(TxOut{script_pubkey: script_pubkey.clone(), value});

    // calculate the size of the tx without an extra cardinal input once it is signed
    let fake_tx = Self::build_fake_transaction(&inputs, &outputs);
    let weight = fake_tx.weight();
    if !self.no_limit && weight > bitcoin::Weight::from_wu(MAX_STANDARD_TX_WEIGHT.into()) {
      bail!(
        "transaction weight greater than {MAX_STANDARD_TX_WEIGHT} (MAX_STANDARD_TX_WEIGHT): {weight}"
      );
    }
    let fee = self.fee_rate.fee(fake_tx.vsize()).to_sat();
    let needed = fee + change_dust_limit;
    let value;
    if cardinal_value < needed {
      // eprintln!("left over amount ({} sats) is too small\n       we need enough for fee {} plus dust limit {} = {} sats", cardinal_value, fee, change_dust_limit, needed);

      let (cardinal_outpoint, new_cardinal_value) = match self.cardinal {
        Some(cardinal) => (cardinal, unspent_outputs[&cardinal].to_sat()),
        None => {
          // select the biggest cardinal - this could be improved by figuring out what size we need, and picking the next biggest for example
          // get a list of available unlocked cardinals
          let cardinals = Self::get_cardinals(unspent_outputs.clone(), locked_outputs, inscriptions);

          if cardinals.is_empty() {
            bail!("wallet has no cardinals");
          }

          cardinals[0]
        }
      };

      // eprintln!("we have {} left over, and {} in the biggest cardinal", cardinal_value, new_cardinal_value);

      // use the biggest cardinal as the last input
      inputs.push(cardinal_outpoint);

      // calculate the size of the tx once it is signed
      let fake_tx = Self::build_fake_transaction(&inputs, &outputs);
      let weight = fake_tx.weight();
      if !self.no_limit && weight > bitcoin::Weight::from_wu(MAX_STANDARD_TX_WEIGHT.into()) {
        bail!(
          "transaction weight greater than {MAX_STANDARD_TX_WEIGHT} (MAX_STANDARD_TX_WEIGHT): {weight}"
        );
      }
      let fee = self.fee_rate.fee(fake_tx.vsize()).to_sat();
      let needed = fee + change_dust_limit;
      if cardinal_value + new_cardinal_value < needed {
        bail!("cardinal {} ({} sats) is too small\n       we need enough for fee {} plus dust limit {} = {} sats",
              cardinal_outpoint.to_string(), new_cardinal_value, fee, change_dust_limit, needed - cardinal_value);
      }
      value = cardinal_value + new_cardinal_value - fee;
    } else {
      value = cardinal_value - fee;
    }

    let last = outputs.len() - 1;
    outputs[last] = TxOut{script_pubkey, value};

    let tx = Self::build_transaction(&inputs, &outputs);

    let signed_tx = client.sign_raw_transaction_with_wallet(&tx, None, None)?;
    let signed_tx = signed_tx.hex;

    if self.broadcast {
      let txid = client.send_raw_transaction(&signed_tx)?.to_string();
      Ok(Box::new(Output { tx: txid }))
    } else {
      Ok(Box::new(Output { tx: signed_tx.raw_hex() }))
    }
  }

  fn get_change_pubkey(
    client: &Client,
    chain: Chain,
    change: Option<Address<NetworkUnchecked>>,
  ) -> Result<ScriptBuf> {
    Ok(match change {
      Some(change) => change.require_network(chain.network()).unwrap(),
      None => get_change_address(&client, chain)?,
    }.script_pubkey())
  }

  fn get_cardinals(
    unspent_outputs: BTreeMap<OutPoint, Amount>,
    locked_outputs: BTreeSet<OutPoint>,
    inscriptions: BTreeMap<InscriptionId, SatPoint>,
  ) -> Vec<(OutPoint, u64)> {
    let inscribed_utxos =
      inscriptions				// get a tree <InscriptionId, SatPoint> of the inscriptions we own
      .values()					// just the SatPoints
      .map(|satpoint| satpoint.outpoint)	// just the OutPoints of those SatPoints
      .collect::<BTreeSet<OutPoint>>();		// as a set of OutPoints

    let mut cardinal_utxos = unspent_outputs
      .iter()
      .filter_map(|(output, amount)| {
        if inscribed_utxos.contains(output) || locked_outputs.contains(output) {
          None
        } else {
          Some((
            *output,
            amount.to_sat(),
          ))
        }
      })
      .collect::<Vec<(OutPoint, u64)>>();

    cardinal_utxos.sort_by_key(|x| x.1);
    cardinal_utxos.reverse();
    cardinal_utxos
  }

  fn build_transaction(
    inputs: &Vec<OutPoint>,
    outputs: &Vec<TxOut>,
  ) -> Transaction {
    Transaction {
      input: inputs
        .iter()
        .map(|outpoint| TxIn {
          previous_output: *outpoint,
          script_sig: script::Builder::new().into_script(),
          witness: Witness::new(),
          sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
        })
        .collect(),
      output: outputs.clone(),
      lock_time: LockTime::ZERO,
      version: 1,
    }
  }

  fn build_fake_transaction(
    inputs: &Vec<OutPoint>,
    outputs: &Vec<TxOut>,
  ) -> Transaction {
    Transaction {
      input: (0..inputs.len())
        .map(|_| TxIn {
          previous_output: OutPoint::null(),
          script_sig: ScriptBuf::new(),
          sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
          witness: Witness::from_slice(&[&[0; Self::SCHNORR_SIGNATURE_SIZE]]),
        })
        .collect(),
      output: outputs.clone(),
      lock_time: LockTime::ZERO,
      version: 1,
    }
  }
}
