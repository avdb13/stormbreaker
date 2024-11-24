#![allow(dead_code)]
pub struct ThreepidValidationSession {
  pub session_id: String,
  pub medium: String,
  pub address: String,
  pub client_secret: String,
  pub last_send_attempt: i64,
  pub validated_at: Option<i64>,
}
