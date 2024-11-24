#![allow(dead_code)]
pub struct AppserviceStreamPosition {
  pub lock: String,
  pub stream_ordering: Option<i64>,
}
