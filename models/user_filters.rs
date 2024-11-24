#![allow(dead_code)]
pub struct UserFilters {
  pub user_id: String,
  pub filter_id: i64,
  pub filter_json: Vec<u8>,
}
