#![allow(dead_code)]
pub struct UserDirectory {
  pub user_id: String,
  pub room_id: Option<String>,
  pub display_name: Option<String>,
  pub avatar_url: Option<String>,
}
