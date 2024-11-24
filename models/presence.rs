#![allow(dead_code)]
pub struct Presence {
  pub user_id: String,
  pub state: Option<String>,
  pub status_msg: Option<String>,
  pub mtime: Option<i64>,
}
