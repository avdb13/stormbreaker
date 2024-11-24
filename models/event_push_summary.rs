#![allow(dead_code)]
pub struct EventPushSummary {
  pub user_id: String,
  pub room_id: String,
  pub notif_count: i64,
  pub stream_ordering: i64,
  pub unread_count: Option<i64>,
  pub last_receipt_stream_ordering: Option<i64>,
  pub thread_id: Option<String>,
}
