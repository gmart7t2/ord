use {super::*, crate::wallet::Wallet};

#[derive(Serialize, Deserialize)]
pub struct Output {
  pub sat: Sat,
  pub number: u64,
  pub inscription: InscriptionId,
  pub location: SatPoint,
  pub explorer: String,
}

pub(crate) fn run(options: Options) -> Result {
  let index = Index::open(&options)?;
  index.update()?;

  let inscriptions = index.get_inscriptions(None)?;
  let unspent_outputs = index.get_unspent_outputs(Wallet::load(&options)?)?;

  let explorer = match options.chain() {
    Chain::Mainnet => "https://ordinals.com/inscription/",
    Chain::Regtest => "http://localhost/inscription/",
    Chain::Signet => "https://signet.ordinals.com/inscription/",
    Chain::Testnet => "https://testnet.ordinals.com/inscription/",
  };

  let mut output = Vec::new();

  for (location, inscription) in inscriptions {
    if unspent_outputs.contains_key(&location.outpoint) {
      let entry = index
        .get_inscription_entry(inscription)?
        .ok_or_else(|| anyhow!("Inscription {inscription} not found"))?;
      output.push(Output {
        sat: entry.sat.unwrap(),
        number: entry.number,
        location,
        inscription,
        explorer: format!("{explorer}{inscription}"),
      });
    }
  }

  print_json(&output)?;

  Ok(())
}
