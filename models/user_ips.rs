#![allow(dead_code)]
pub struct UserIps {
  pub user_id: String,
  pub access_token: String,
  pub device_id: Option<String>,
  pub ip: String,
  pub user_agent: String,
  pub last_seen: i64,
}
