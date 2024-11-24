#![allow(dead_code)]
pub struct UserDailyVisits {
  pub user_id: String,
  pub device_id: Option<String>,
  pub timestamp: i64,
  pub user_agent: Option<String>,
}
