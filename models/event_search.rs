#![allow(dead_code)]
pub struct EventSearch {
  pub event_id: Option<String>,
  pub room_id: Option<String>,
  pub sender: Option<String>,
  pub key: Option<String>,
  pub vector: Option<Vec<String>>,
  pub origin_server_ts: Option<i64>,
  pub stream_ordering: Option<i64>,
}
