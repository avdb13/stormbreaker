#![allow(dead_code)]
pub struct ExOutlierStream {
  pub event_stream_ordering: i64,
  pub event_id: String,
  pub state_group: i64,
  pub instance_name: Option<String>,
}
