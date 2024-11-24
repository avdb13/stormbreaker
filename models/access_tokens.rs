#![allow(dead_code)]
pub struct AccessTokens {
  pub id: i64,
  pub user_id: String,
  pub device_id: Option<String>,
  pub token: String,
  pub valid_until_ms: Option<i64>,
  pub puppets_user_id: Option<String>,
  pub last_validated: Option<i64>,
  pub refresh_token_id: Option<i64>,
  pub used: Option<bool>,
}
