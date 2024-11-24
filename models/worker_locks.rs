#![allow(dead_code)]
pub struct WorkerLocks {
  pub lock_name: String,
  pub lock_key: String,
  pub instance_name: String,
  pub token: String,
  pub last_renewed_ts: i64,
}
