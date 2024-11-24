#![allow(dead_code)]
pub struct PushRules {
  pub id: i64,
  pub user_name: String,
  pub rule_id: String,
  pub priority_class: i16,
  pub priority: i32,
  pub conditions: String,
  pub actions: String,
}
