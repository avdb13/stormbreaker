#![allow(dead_code)]
pub struct DeviceInbox {
  pub user_id: String,
  pub device_id: String,
  pub stream_id: i64,
  pub message_json: String,
  pub instance_name: Option<String>,
}
