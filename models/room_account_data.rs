#![allow(dead_code)]
pub struct RoomAccountData {
  pub user_id: String,
  pub room_id: String,
  pub account_data_type: String,
  pub stream_id: i64,
  pub content: String,
  pub instance_name: Option<String>,
}
