use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Restore {
  #[arg(help = "Restore wallet from <MNEMONIC>")]
  mnemonic: Mnemonic,
  #[arg(
    long,
    default_value = "",
    help = "Use <PASSPHRASE> when deriving wallet"
  )]
  pub(crate) passphrase: String,
  #[arg(long, value_enum, default_value="bech32m")]
  pub(crate) address_type: AddressType,
}

impl Restore {
  pub(crate) fn run(self, options: Options) -> SubcommandResult {
    initialize_wallet(&options, self.mnemonic.to_seed(self.passphrase), self.address_type)?;
    Ok(Box::new(Empty {}))
  }
}
