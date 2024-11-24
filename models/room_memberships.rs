#![allow(dead_code)]
pub struct RoomMemberships {
  pub event_id: String,
  pub user_id: String,
  pub sender: String,
  pub room_id: String,
  pub membership: String,
  pub forgotten: Option<i32>,
  pub display_name: Option<String>,
  pub avatar_url: Option<String>,
}
