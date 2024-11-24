#![allow(dead_code)]
pub struct StateEvents {
  pub event_id: String,
  pub room_id: String,
  pub type: String,
  pub state_key: String,
  pub prev_state: Option<String>,
}
