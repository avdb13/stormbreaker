#![allow(dead_code)]
pub struct E2eRoomKeys {
  pub user_id: String,
  pub room_id: String,
  pub session_id: String,
  pub version: i64,
  pub first_message_index: Option<i32>,
  pub forwarded_count: Option<i32>,
  pub is_verified: Option<bool>,
  pub session_data: String,
}
