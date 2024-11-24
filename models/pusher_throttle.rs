#![allow(dead_code)]
pub struct PusherThrottle {
  pub pusher: i64,
  pub room_id: String,
  pub last_sent_ts: Option<i64>,
  pub throttle_ms: Option<i64>,
}
