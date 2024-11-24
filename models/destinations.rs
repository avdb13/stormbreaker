#![allow(dead_code)]
pub struct Destinations {
  pub destination: String,
  pub retry_last_ts: Option<i64>,
  pub retry_interval: Option<i64>,
  pub failure_ts: Option<i64>,
  pub last_successful_stream_ordering: Option<i64>,
}
