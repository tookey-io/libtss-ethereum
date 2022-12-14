use napi_derive::napi;
use tookey_libtss::keygen::{KeygenParams, KeygenResult};
use tookey_libtss::sign::{SignParams, SignResult};

///
/// Re-Exports
///

#[napi]
#[allow(dead_code)]
pub fn get_version() -> String {
  env!("CARGO_PKG_VERSION").to_owned()
}

#[napi]
#[allow(dead_code)]
pub async fn keygen(params: KeygenParams) -> KeygenResult {
  tookey_libtss::keygen::keygen(params).await
}

#[napi]
#[allow(dead_code)]
pub async fn sign(params: SignParams) -> SignResult {
  tookey_libtss::sign::sign(params).await
}

///
/// Ethers
///

#[napi(object)]
pub struct EthersResult {
  pub result: Option<String>,
  pub error: Option<String>,
}

#[napi]
#[allow(dead_code)]
pub fn private_key_to_ethereum_address(private_key: String) -> EthersResult {
  match crate::ethers::private_key_to_ethereum_address(private_key) {
    Ok(val) => EthersResult {
      result: Some(val),
      error: None,
    },
    Err(err) => EthersResult {
      result: None,
      error: Some(format!("{:?}", err)),
    },
  }
}

#[napi]
#[allow(dead_code)]
pub fn private_key_to_public_key(private_key: String, compressed: Option<bool>) -> EthersResult {
  match crate::ethers::private_key_to_public_key(private_key, compressed) {
    Ok(val) => EthersResult {
      result: Some(val),
      error: None,
    },
    Err(err) => EthersResult {
      result: None,
      error: Some(format!("{:?}", err)),
    },
  }
}

#[napi]
#[allow(dead_code)]
pub fn encode_message_signature(message_hash: String, chain_id: u32, signature_recid: String) -> EthersResult {
  match crate::ethers::encode_message_signature(message_hash, chain_id, signature_recid) {
    Ok(val) => EthersResult {
      result: Some(val),
      error: None,
    },
    Err(err) => EthersResult {
      result: None,
      error: Some(format!("{:?}", err)),
    },
  }
}
