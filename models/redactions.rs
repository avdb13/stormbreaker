#![allow(dead_code)]
pub struct Redactions {
  pub event_id: String,
  pub redacts: String,
  pub have_censored: bool,
  pub received_ts: Option<i64>,
}
