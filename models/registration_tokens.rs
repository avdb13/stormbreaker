#![allow(dead_code)]
pub struct RegistrationTokens {
  pub token: String,
  pub uses_allowed: Option<i32>,
  pub pending: i32,
  pub completed: i32,
  pub expiry_time: Option<i64>,
}
