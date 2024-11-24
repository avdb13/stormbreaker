#![allow(dead_code)]
pub struct DeviceFederationInbox {
  pub origin: String,
  pub message_id: String,
  pub received_ts: i64,
  pub instance_name: Option<String>,
}
