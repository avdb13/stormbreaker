#![allow(dead_code)]
pub struct E2eRoomKeysVersions {
  pub user_id: String,
  pub version: i64,
  pub algorithm: String,
  pub auth_data: String,
  pub deleted: i16,
  pub etag: Option<i64>,
}
