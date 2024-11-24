#![allow(dead_code)]
pub struct Rooms {
  pub room_id: String,
  pub is_public: Option<bool>,
  pub creator: Option<String>,
  pub room_version: Option<String>,
  pub has_auth_chain_index: Option<bool>,
}
