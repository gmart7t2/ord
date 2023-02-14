use {
  anyhow::{anyhow, Result},
  bitcoin::{consensus::deserialize, BlockHash, Transaction, Txid},
  hyper::{client::HttpConnector, Client, Uri},
  std::{str::FromStr, time::Duration},
  tokio::time::sleep,
};

const MAX_CONNECTIONS: usize = 100;
const RETRY_COUNT: usize = 3;

pub(crate) struct Rest {
  client: Client<HttpConnector>,
  url: String,
}

impl Rest {
  pub(crate) fn new(url: &str) -> Self {
    let url = if !url.starts_with("http://") {
      "http://".to_string() + url
    } else {
      url.to_string()
    };
    let client = Client::builder()
      .pool_max_idle_per_host(MAX_CONNECTIONS)
      .build_http();
    Rest { client, url }
  }

  pub(crate) async fn get_block_hash(&self, height: u32) -> Result<BlockHash> {
    let url = format!("{}/rest/blockhashbyheight/{height}.bin", self.url);
    let res = self.client.get(Uri::from_str(&url)?).await?;
    let buf = hyper::body::to_bytes(res).await?;
    let block_hash = deserialize(&buf)?;
    Ok(block_hash)
  }

  pub(crate) async fn get_raw_transaction(&self, txid: &Txid) -> Result<Transaction> {
    for i in 0..RETRY_COUNT {
      let res: Result<Transaction> = async {
        let url = format!("{}/rest/tx/{txid:x}.bin", self.url);
        let res = self.client.get(Uri::from_str(&url)?).await?;
        let buf = hyper::body::to_bytes(res).await?;
        let tx = deserialize(&buf)?;
        Ok(tx)
      }
      .await;
      match res {
        Ok(res) => {
          return Ok(res);
        }
        Err(_) => {
          let duration = 2 ^ i;
          sleep(Duration::from_secs(duration as u64)).await;
        }
      };
    }
    return Err(anyhow!("Could not fetch tx {txid:x}"));
  }
}
