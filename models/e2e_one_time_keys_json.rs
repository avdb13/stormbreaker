#![allow(dead_code)]
pub struct E2eOneTimeKeysJson {
  pub user_id: String,
  pub device_id: String,
  pub algorithm: String,
  pub key_id: String,
  pub ts_added_ms: i64,
  pub key_json: String,
}
