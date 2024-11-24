#![allow(dead_code)]
pub struct RoomTagsRevisions {
  pub user_id: String,
  pub room_id: String,
  pub stream_id: i64,
  pub instance_name: Option<String>,
}
