#![allow(dead_code)]
pub struct PushRulesEnable {
  pub id: i64,
  pub user_name: String,
  pub rule_id: String,
  pub enabled: Option<i16>,
}
