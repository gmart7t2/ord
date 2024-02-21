use {
  super::*,
  chrono::NaiveDateTime,
  std::io::{BufRead, BufReader},
};

#[derive(Debug, Parser)]
pub(crate) struct Find {
  #[arg(long, help = "Only look in specified outpoint(s).")]
  outpoint: Vec<OutPoint>,
  #[arg(
    long,
    default_value = "%Y-%m-%d %H:%M:%S",
    help = "Set the format to use for dates. See 'https://docs.rs/chrono/latest/chrono/format/strftime/'."
  )]
  date_format: String,
  #[arg(
    long,
    help = "Read a list of sats and ranges to find from a file. One sat or range per line. Ranges written as <start>-<end>."
  )]
  file: Vec<PathBuf>,
  #[arg(long, help = "Ignore bad sat ranges.")]
  ignore: bool,
  #[arg(long, help = "Show addresses in the results.")]
  show_address: bool,
  #[arg(long, help = "Show blockhashes in the results.")]
  show_blockhash: bool,
  #[arg(long, help = "Show dates in the results.")]
  show_date: bool,
  #[arg(long, help = "Show block heights in the results.")]
  show_height: bool,
  #[arg(long, help = "Show sat names in the results.")]
  show_name: bool,
  #[arg(long, help = "Show timestamps in the results.")]
  show_time: bool,
  #[arg(long, help = "Show output values in the results.")]
  show_value: bool,
  #[arg(help = "Find output and offset of <SAT>.")]
  sat: Option<Sat>,
  #[arg(help = "Find output and offset of all sats in the range <SAT>-<END>.")]
  end: Option<Sat>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
  pub start: u64,
  pub size: u64,
  pub satpoint: SatPoint,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub address: Option<Address<NetworkUnchecked>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub blockhash: Option<bitcoin::BlockHash>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub date: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub height: Option<usize>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp: Option<usize>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<u64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FindRangeOutput {
  pub start: u64,
  pub size: u64,
  pub satpoint: SatPoint,
}

impl Find {
  pub(crate) fn run(self, options: Options) -> SubcommandResult {
    let index = Index::open(&options)?;

    if !index.has_sat_index() {
      bail!("find requires index created with `--index-sats` flag");
    }

    index.update()?;

    let mut targets = Vec::new();
    let mut results = Vec::new();

    if let Some(sat) = self.sat {
      let end = match self.end {
        Some(end) => end,
        None => sat + 1,
      };

      if sat < end {
        targets.push((sat, end));
      } else {
        bail!("range is empty");
      }
    }

    let comment_re = Regex::new(r"#.*")?;
    for file in self.file {
      let reader = BufReader::new(File::open(&file)?);
      let mut line_number = 0;
      for line in reader.lines() {
        line_number += 1;
        let line = line?;
        let line = comment_re.replace(&line, "");
        let line = line.trim();
        if line.is_empty() {
          continue;
        }
        let mut split = line
          .trim_start_matches('\u{feff}')
          .split(&['-', '\u{2013}']); // ASCII hyphen or Unicode 'EN DASH'
        if let Some(start) = split.next() {
          let start = match start.parse() {
            Ok(start) => start,
            Err(err) => bail!(
              "parsing sat '{start}' on line {line_number} of '{:?}': {err}",
              file
            ),
          };

          let end = match split.next() {
            Some(end) => {
              if let Some(junk) = split.next() {
                bail!(
                  "trailing junk on line {line_number} of '{}' ({})",
                  file.display(),
                  junk
                );
              } else {
                match end.parse() {
                  Ok(end) => end,
                  Err(err) => bail!(
                    "parsing range end '{end}' on line {line_number} of '{:?}': {err}",
                    file
                  ),
                }
              }
            }
            None => start + 1,
          };
          targets.push((start, end));
        } else {
          bail!(
            "file '{}' is not formatted correctly at line {line_number}",
            file.display()
          );
        }
      }
    }

    if targets.is_empty() {
      bail!("nothing to find");
    }

    // loop through targets
    for (sat, end) in targets {
      // eprintln!("find {sat}-{end}");
      match index.find_range(sat, end, &self.outpoint, self.ignore)? {
        Some(result) => {
          // eprintln!("  found {} satpoints", result.len());
          results.extend(result);
        }
        None => {
          if !self.ignore {
            return Err(anyhow!(
              "range {sat}-{end} not found; use --ignore to continue anyway"
            ));
          }
        }
      }
    }

    let mut detailed_results = Vec::new();

    // let gbt = options.chain().genesis_block().coinbase().unwrap().clone();
    // print_json(&gbt)?;
    // println!("gbt.output = {:?}", options.chain().address_from_script(&gbt.output[0].script_pubkey));
    // result.satpoint.outpoint.txid == gbt.txid()

    for result in results {
      let tx = if self.show_address
        || self.show_blockhash
        || self.show_date
        || self.show_height
        || self.show_time
        || self.show_value
      {
        index
          .get_transaction_info(result.satpoint.outpoint.txid)
          .ok()
      } else {
        None
      };

      let mut result = Output {
        start: result.start,
        size: result.size,
        satpoint: result.satpoint,
        address: None,
        blockhash: None,
        date: None,
        height: None,
        name: None,
        timestamp: None,
        value: None,
      };

      if let Some(tx) = tx.clone() {
        if self.show_address {
          result.address = tx.vout[result.satpoint.outpoint.vout as usize]
            .script_pub_key
            .address
            .clone();
        }

        if self.show_blockhash {
          result.blockhash = tx.blockhash;
        }

        if self.show_height {
          result.height = Some(index.get_block_height(tx.blockhash.unwrap())?);
        }

        if self.show_date {
          result.date = Some(
            NaiveDateTime::from_timestamp_opt(tx.time.unwrap() as i64, 0)
              .unwrap()
              .format(&self.date_format)
              .to_string(),
          );
        }

        if self.show_time {
          result.timestamp = tx.time;
        }

        if self.show_value {
          result.value = Some(
            tx.vout[result.satpoint.outpoint.vout as usize]
              .value
              .to_sat(),
          );
        }
      }

      if self.show_name {
        result.name = Some(Sat(result.start).name());
      }

      detailed_results.push(result);
    }

    Ok(Box::new(detailed_results))
  }
}
