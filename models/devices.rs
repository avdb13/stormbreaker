#![allow(dead_code)]
pub struct Devices {
  pub user_id: String,
  pub device_id: String,
  pub display_name: Option<String>,
  pub last_seen: Option<i64>,
  pub ip: Option<String>,
  pub user_agent: Option<String>,
  pub hidden: Option<bool>,
}
