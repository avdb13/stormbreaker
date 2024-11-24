#![allow(dead_code)]
pub struct AccountValidity {
  pub user_id: String,
  pub expiration_ts_ms: i64,
  pub email_sent: bool,
  pub renewal_token: Option<String>,
  pub token_used_ts_ms: Option<i64>,
}
