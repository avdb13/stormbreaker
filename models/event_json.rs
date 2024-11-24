#![allow(dead_code)]
pub struct EventJson {
  pub event_id: String,
  pub room_id: String,
  pub internal_metadata: String,
  pub json: String,
  pub format_version: Option<i32>,
}
