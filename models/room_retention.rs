#![allow(dead_code)]
pub struct RoomRetention {
  pub room_id: String,
  pub event_id: String,
  pub min_lifetime: Option<i64>,
  pub max_lifetime: Option<i64>,
}
