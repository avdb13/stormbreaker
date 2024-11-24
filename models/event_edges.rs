#![allow(dead_code)]
pub struct EventEdges {
  pub event_id: String,
  pub prev_event_id: String,
  pub room_id: Option<String>,
  pub is_state: bool,
}
