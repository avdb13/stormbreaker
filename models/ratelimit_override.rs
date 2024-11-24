#![allow(dead_code)]
pub struct RatelimitOverride {
  pub user_id: String,
  pub messages_per_second: Option<i64>,
  pub burst_count: Option<i64>,
}
