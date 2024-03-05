use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Inscribedsats {
  #[arg(help = "List inscriptions on sats in range starting with <START>.")]
  start: Sat,
  #[arg(help = "List inscriptions on sats in range ending with <END>.")]
  end: Sat,
}

impl Inscribedsats {
  pub(crate) fn run(self, options: Options) -> SubcommandResult {
    let index = Index::open(&options)?;

    if !index.has_sat_index() {
      bail!("list requires index created with `--index-sats` flag");
    }

    index.update()?;

    Ok(Box::new(index.get_inscription_ids_by_sat_range(self.start, self.end)?))
  }
}
