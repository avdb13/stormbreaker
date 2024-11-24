#![allow(dead_code)]
pub struct RoomStatsCurrent {
  pub room_id: String,
  pub current_state_events: i32,
  pub joined_members: i32,
  pub invited_members: i32,
  pub left_members: i32,
  pub banned_members: i32,
  pub local_users_in_room: i32,
  pub completed_delta_stream_id: i64,
  pub knocked_members: Option<i32>,
}
