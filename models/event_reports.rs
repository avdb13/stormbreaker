#![allow(dead_code)]
pub struct EventReports {
  pub id: i64,
  pub received_ts: i64,
  pub room_id: String,
  pub event_id: String,
  pub user_id: String,
  pub reason: Option<String>,
  pub content: Option<String>,
}
