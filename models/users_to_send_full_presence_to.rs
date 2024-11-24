#![allow(dead_code)]
pub struct UsersToSendFullPresenceTo {
  pub user_id: String,
  pub presence_stream_id: Option<i64>,
}
