#![allow(dead_code)]
pub struct DeviceFederationOutbox {
  pub destination: String,
  pub stream_id: i64,
  pub queued_ts: i64,
  pub messages_json: String,
  pub instance_name: Option<String>,
}
