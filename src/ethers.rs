use crate::tx::{Signature, SignatureRecid, Transaction};
use anyhow::Context;
use hex::ToHex;
use std::ops::Deref;
use tookey_libtss::curv::arithmetic::Integer;
use tookey_libtss::curv::elliptic::curves::secp256_k1::{Secp256k1Point, Secp256k1Scalar};
use tookey_libtss::curv::elliptic::curves::{ECPoint, ECScalar, Scalar, Secp256k1};
use tookey_libtss::curv::BigInt;
use tookey_libtss::ecdsa::state_machine::keygen::LocalKey;
use web3::{
  ethabi::Address,
  signing::keccak256,
  types::{Recovery, H256},
};

/// Encode transaction and hash
pub fn transaction_to_message_hash(tx_request: String) -> anyhow::Result<String> {
  let transaction: Transaction = serde_json::from_str(&tx_request)?;

  let message = transaction.encode(None);
  message_to_hash(String::from("0x") + &hex::encode(message))
}

/// Hash message
pub fn message_to_hash(data: String) -> anyhow::Result<String> {
  let message = bytes_from_hex(data)?;
  let message_hash = H256::from(keccak256(&message));

  Ok(String::from("0x") + &message_hash.encode_hex::<String>())
}

/// Convert SignatureRecid (from sign method) to Ethereum signature
pub fn encode_message_signature(
  message_hash: String,
  chain_id: u32,
  signature_recid: String,
) -> anyhow::Result<String> {
  let mut signature: SignatureRecid = serde_json::from_str(&signature_recid)?;

  sanitize_signature(&mut signature, chain_id);

  let rec = Recovery::new(
    bytes_from_hex(message_hash)?,
    signature.recid,
    H256::from_slice(signature.r.to_bytes().as_ref()),
    H256::from_slice(signature.s.to_bytes().as_ref()),
  );

  let (signature, v) = rec.as_signature().context("failed take signature from recoverable")?;

  let mut slice: [u8; 65] = [0u8; 65];

  slice[..64].copy_from_slice(&signature);
  slice[64] = v as u8;

  Ok(String::from("0x") + &ethereum_types::Signature::from_slice(&slice).encode_hex::<String>())
}

/// Encode Transaction with SignatureRecid to send with eth_sendRawTransaction
pub fn encode_transaction(tx_request: String, signature_recid: String) -> anyhow::Result<Vec<u8>> {
  let transaction: Transaction = serde_json::from_str(&tx_request)?;
  let mut signature: SignatureRecid = serde_json::from_str(&signature_recid)?;

  sanitize_signature(&mut signature, transaction.chain_id.as_u32());

  let sig = Signature {
    v: signature.recid,
    r: H256::from_slice(&signature.r.to_bytes()),
    s: H256::from_slice(&signature.s.to_bytes()),
  };

  Ok(transaction.encode(Some(&sig)))
}

pub fn public_key_to_ethereum_address(public_key: String) -> anyhow::Result<String> {
  let buffer = bytes_from_hex(public_key)?;
  let point = Secp256k1Point::deserialize(&buffer)?.serialize_uncompressed();

  let hash = keccak256(&point[1..]);
  Ok(checksum(Address::from_slice(&hash[12..])))
}

/// Convert Tookey private key to ethereum public address
pub fn private_key_to_ethereum_address(private_key: String) -> anyhow::Result<String> {
  let key: LocalKey<Secp256k1> = serde_json::from_str(&private_key)?;

  let public_key = key.y_sum_s.as_raw().serialize_uncompressed();

  debug_assert_eq!(public_key[0], 0x04);
  let hash = keccak256(&public_key[1..]);

  Ok(checksum(Address::from_slice(&hash[12..])))
}

/// Convert Tookey private key to Tookey public key
pub fn private_key_to_public_key(private_key: String, compressed: Option<bool>) -> anyhow::Result<String> {
  let key: LocalKey<Secp256k1> = serde_json::from_str(&private_key)?;

  Ok(
    key
      .public_key()
      .to_bytes(compressed.unwrap_or(true))
      .deref()
      .encode_hex(),
  )
}

fn bytes_from_hex(data: String) -> anyhow::Result<Vec<u8>> {
  let result = if data.starts_with("0x") {
    hex::decode(data.strip_prefix("0x").unwrap())?
  } else {
    hex::decode(data)?
  };

  Ok(result)
}

fn sanitize_signature(signature: &mut SignatureRecid, chain: u32) {
  let s = signature.s.to_bigint();
  let n = Secp256k1Scalar::group_order().clone();
  let half_n = n.div_floor(&BigInt::from(2));
  if s > half_n {
    let ns = n - s;
    signature.s = Scalar::<Secp256k1>::from(&ns);
  }

  if signature.recid <= 3 {
    signature.recid += (chain as u64) * 2 + 35;
  }
}

/// Gets the checksummed address of a H160 hash
fn checksum(address: Address) -> String {
  let address = format!("{:x}", address);
  let address_hash = format!("{:x}", H256::from(keccak256(address.as_bytes())));

  address
    .char_indices()
    .fold(String::from("0x"), |mut acc, (index, address_char)| {
      let n = u16::from_str_radix(&address_hash[index..index + 1], 16).unwrap();

      if n > 7 {
        // make char uppercase if ith character is 9..f
        acc.push_str(&address_char.to_uppercase().to_string())
      } else {
        // already lowercased
        acc.push(address_char)
      }

      acc
    })
}
#[cfg(test)]
pub mod test {
  use super::public_key_to_ethereum_address;

  #[test]
  fn address_from_public_key() {
    let expected = "0x6cc93958DA6Bf5de40A15935A53eabB6695AaF9e";
    let public_key = "02df6a1f98f0c2ba133f928de2f7cc4b0c595afbe6b5a4cd3e56f6c7a5bcd5f19f";

    let calculated = public_key_to_ethereum_address(public_key.to_owned()).unwrap();

    assert_eq!(calculated, expected);
  }
}
