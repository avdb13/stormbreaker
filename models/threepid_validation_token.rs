#![allow(dead_code)]
pub struct ThreepidValidationToken {
  pub token: String,
  pub session_id: String,
  pub next_link: Option<String>,
  pub expires: i64,
}
