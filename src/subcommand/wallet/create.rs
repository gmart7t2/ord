use super::*;

#[derive(Serialize, Deserialize)]
pub struct Output {
  pub mnemonic: Mnemonic,
  pub passphrase: Option<String>,
}

#[derive(Debug, Parser)]
pub(crate) struct Create {
  #[arg(
    long,
    default_value = "",
    help = "Use <PASSPHRASE> to derive wallet seed."
  )]
  pub(crate) passphrase: String,
  #[arg(long, value_enum, default_value="bech32m")]
  pub(crate) address_type: AddressType,
}

impl Create {
  pub(crate) fn run(self, options: Options) -> SubcommandResult {
    let mut entropy = [0; 16];
    rand::thread_rng().fill_bytes(&mut entropy);

    let mnemonic = Mnemonic::from_entropy(&entropy)?;

    initialize_wallet(&options, mnemonic.to_seed(self.passphrase.clone()), self.address_type)?;

    Ok(Box::new(Output {
      mnemonic,
      passphrase: Some(self.passphrase),
    }))
  }
}
