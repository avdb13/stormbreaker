#![allow(dead_code)]
pub struct CurrentStateEvents {
  pub event_id: String,
  pub room_id: String,
  pub type: String,
  pub state_key: String,
  pub membership: Option<String>,
}
