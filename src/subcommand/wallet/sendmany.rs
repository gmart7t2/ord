use {
  super::*,
  crate::wallet::Wallet,
  bitcoin::{
    locktime::absolute::LockTime,
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
  #[arg(long, help = "Use fee rate of <FEE_RATE> sats/vB")]
  fee_rate: FeeRate,
  #[clap(long, help = "Location of a CSV file containing `inscriptionid`,`destination` pairs.")]
  pub(crate) csv: PathBuf,
  #[clap(long, help = "Broadcast the transaction; the default is to output the raw tranasction hex so you can check it before broadcasting.")]
  pub(crate) broadcast: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Output {
  pub tx: String,
}

impl SendMany {
  const SCHNORR_SIGNATURE_SIZE: usize = 64;

  pub(crate) fn run(self, options: Options) -> SubcommandResult {
    let file = File::open(&self.csv)?;
    let reader = BufReader::new(file);
    let mut line_number = 1;
    let mut requested = BTreeMap::new();

    let chain = options.chain();

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

    let client = options.bitcoin_rpc_client_for_wallet_command(false)?;
    let unspent_outputs = index.get_unspent_outputs(Wallet::load(&options)?)?;
    let locked_outputs = index.get_locked_outputs(Wallet::load(&options)?)?;

    // we get a tree <SatPoint, InscriptionId>, and turn it into
    //        a tree <InscriptionId, SatPoint>
    let mut inscriptions = BTreeMap::new();
    for (satpoint, inscriptionid) in index.get_inscriptions(&unspent_outputs)? {
      inscriptions.insert(inscriptionid, satpoint);
    }

    let mut ordered_inscriptions = Vec::new();
    let mut total_value = 0;
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();

    while !requested.is_empty() {
      let mut inscriptions_on_outpoint = Vec::new();
      for (inscriptionid, _address) in &requested {
        if !inscriptions.contains_key(&inscriptionid) {
          bail!("inscriptionid {} isn't in the wallet", inscriptionid.to_string());
        }

        let satpoint = inscriptions[inscriptionid];
        let outpoint = satpoint.outpoint;
        inscriptions_on_outpoint = index.get_inscriptions_on_output_with_satpoints(outpoint)?;
        inscriptions_on_outpoint.sort_by_key(|(s, _)| s.offset);
        for (_satpoint, outpoint_inscriptionid) in &inscriptions_on_outpoint {
          if !requested.contains_key(&outpoint_inscriptionid) {
            bail!("inscriptionid {} is in the same output as {} but wasn't in the CSV file", outpoint_inscriptionid.to_string(), inscriptionid.to_string());
          }
        }
        break;
      }

      let (first_satpoint, _first_inscription) = inscriptions_on_outpoint[0];
      let first_offset = first_satpoint.offset;
      let first_outpoint = first_satpoint.outpoint;
      let utxo_value = unspent_outputs[&first_outpoint].to_sat();

      if first_offset != 0 {
        bail!("the first inscription in {} is at non-zero offset {}", first_outpoint, first_offset);
      }

      eprintln!("\noutput {}, worth {}:", first_outpoint, utxo_value);
      total_value += utxo_value;

      inputs.push(first_outpoint);

      for (i, (satpoint, inscriptionid)) in inscriptions_on_outpoint.iter().enumerate() {
        let destination = &requested[inscriptionid];
        let offset = satpoint.offset;
        let value = if i == inscriptions_on_outpoint.len() - 1 {
          utxo_value - offset
        } else {
          inscriptions_on_outpoint[i + 1].0.offset - offset
        };
        let script_pubkey = destination.script_pubkey();
        let dust_limit = script_pubkey.dust_value().to_sat();
        if value < dust_limit {
          bail!("inscription {} at {} is only followed by {} sats, less than dust limit {} for address {}",
                inscriptionid, satpoint.to_string(), value, dust_limit, destination);
        }

        eprintln!("  {} : offset: {}, value: {}\n          id: {}\n        dest: {}", i, offset, value, inscriptionid, destination);
        outputs.push(TxOut{script_pubkey, value});
        requested.remove(&inscriptionid);
      }
      ordered_inscriptions.extend(inscriptions_on_outpoint);
    }

    let cardinals = Self::get_cardinals(unspent_outputs, locked_outputs, inscriptions);

    if cardinals.is_empty() {
      bail!("wallet has no cardinals");
    }

    // select the biggest cardinal - this could be improved by figuring out what size we need, and picking the next biggest for example
    let (cardinal_outpoint, cardinal_value) = cardinals[0];
    eprintln!("\ncardinal:\n  {}, worth {}", cardinal_outpoint.to_string(), cardinal_value);

    eprintln!("\ninputs without cardinal: {}", total_value);
    total_value += cardinal_value;
    eprintln!("inputs with cardinal: {}", total_value);

    inputs.push(cardinal_outpoint);

    let change_address = get_change_address(&client, chain)?;
    let script_pubkey = change_address.script_pubkey();
    let dust_limit = script_pubkey.dust_value().to_sat();
    let value = 0;
    outputs.push(TxOut{script_pubkey: script_pubkey.clone(), value});

    // calculate the size of the tx once it is signed
    let vsize = Self::estimate_transaction_vsize(inputs.len(), outputs.clone());
    let fee = self.fee_rate.fee(vsize).to_sat();
    let needed = fee + dust_limit;
    if cardinal_value < needed {
      bail!("cardinal ({}) is too small: we need enough for fee {} plus dust limit {} = {}", cardinal_value, fee, dust_limit, needed);
    }
    let value = cardinal_value - fee;
    eprintln!("vsize: {}, fee: {}, change: {}\n", vsize, fee, value);
    let last = outputs.len() - 1;
    outputs[last] = TxOut{script_pubkey, value};

    let tx = Self::build_transaction(inputs, outputs);

    let signed_tx = client.sign_raw_transaction_with_wallet(&tx, None, None)?;
    let signed_tx = signed_tx.hex;

    if self.broadcast {
      let txid = client.send_raw_transaction(&signed_tx)?.to_string();
      Ok(Box::new(Output { tx: txid }))
    } else {
      Ok(Box::new(Output { tx: signed_tx.raw_hex() }))
    }
  }

  fn get_cardinals(
    unspent_outputs: BTreeMap<OutPoint, Amount>,
    locked_outputs: BTreeSet<OutPoint>,
    inscriptions: BTreeMap<InscriptionId, SatPoint>,
  ) -> Vec<(OutPoint, u64)> {
    let inscribed_utxos =
      inscriptions				// get a tree <InscriptionId, SatPoint> of the inscriptions we own
      .values()					// just the SatPoints
      .map(|satpoint| satpoint.outpoint)		// just the OutPoints of those SatPoints
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
    inputs: Vec<OutPoint>,
    outputs: Vec<TxOut>,
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
      output: outputs,
      lock_time: LockTime::ZERO,
      version: 1,
    }
  }

  fn estimate_transaction_vsize(
    inputs: usize,
    outputs: Vec<TxOut>,
  ) -> usize {
    Transaction {
      input: (0..inputs)
        .map(|_| TxIn {
          previous_output: OutPoint::null(),
          script_sig: ScriptBuf::new(),
          sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
          witness: Witness::from_slice(&[&[0; Self::SCHNORR_SIGNATURE_SIZE]]),
        })
        .collect(),
      output: outputs,
      lock_time: LockTime::ZERO,
      version: 1,
    }.vsize()
  }
}
