#![allow(dead_code)]
pub struct ServerSignatureKeys {
  pub server_name: Option<String>,
  pub key_id: Option<String>,
  pub from_server: Option<String>,
  pub ts_added_ms: Option<i64>,
  pub verify_key: Option<Vec<u8>>,
  pub ts_valid_until_ms: Option<i64>,
}
