#![allow(dead_code)]
pub struct CurrentStateDeltaStream {
  pub stream_id: i64,
  pub room_id: String,
  pub type: String,
  pub state_key: String,
  pub event_id: Option<String>,
  pub prev_event_id: Option<String>,
  pub instance_name: Option<String>,
}
