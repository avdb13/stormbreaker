#![allow(dead_code)]
pub struct RefreshTokens {
  pub id: i64,
  pub user_id: String,
  pub device_id: String,
  pub token: String,
  pub next_token_id: Option<i64>,
  pub expiry_ts: Option<i64>,
  pub ultimate_session_expiry_ts: Option<i64>,
}
