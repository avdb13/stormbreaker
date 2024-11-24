#![allow(dead_code)]
pub struct ServerKeysJson {
  pub server_name: String,
  pub key_id: String,
  pub from_server: String,
  pub ts_added_ms: i64,
  pub ts_valid_until_ms: i64,
  pub key_json: Vec<u8>,
}
