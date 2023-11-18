use {
  super::*,
  crate::wallet::Wallet,
  std::io::{BufRead, BufReader},
  std::fs::File,
};

#[derive(Debug, Parser, Clone)]
pub(crate) struct SendMany {
  #[arg(long, help = "Use fee rate of <FEE_RATE> sats/vB")]
  fee_rate: FeeRate,
  #[clap(long, help = "Location of a CSV file containing `inscriptionid`,`destination` pairs.")]
  pub(crate) csv: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct Output {
  pub transaction: Txid,
}

impl SendMany {
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

    let _client = options.bitcoin_rpc_client_for_wallet_command(false)?;

    let unspent_outputs = index.get_unspent_outputs(Wallet::load(&options)?)?;

    let _locked_outputs = index.get_locked_outputs(Wallet::load(&options)?)?;

    let mut inscriptions = BTreeMap::new();
    for (satpoint, inscriptionid) in index.get_inscriptions(&unspent_outputs)? {
      inscriptions.insert(inscriptionid, satpoint);
    }

    let mut ordered_inscriptions = Vec::new();
    let mut total_value = Amount::from_sat(0);

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
      let utxo_value = unspent_outputs[&first_outpoint];

      if first_offset != 0 {
        bail!("the first inscription in {} is at non-zero offset {}", first_outpoint, first_offset);
      }

      println!("using output {} worth {}", first_outpoint, utxo_value.to_sat());
      total_value += utxo_value;

      for (_satpoint, inscriptionid) in &inscriptions_on_outpoint {
        requested.remove(&inscriptionid);
      }
      ordered_inscriptions.extend(inscriptions_on_outpoint);
    }

    println!("\ntotal inputs before cardinal: {}", total_value);

    println!("\nsending these inscriptions, in this order:");
    for (satpoint, inscriptionid) in &ordered_inscriptions {
      println!("  inscriptionid {}, satpoint {}", inscriptionid.to_string(), satpoint.to_string())
    }
    println!("");

    let txid = Txid::from_str("0000000000000000000000000000000000000000000000000000000000000000")?;
    Ok(Box::new(Output { transaction: txid }))
  }
}
