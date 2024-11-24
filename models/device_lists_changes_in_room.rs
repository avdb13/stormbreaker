#![allow(dead_code)]
pub struct DeviceListsChangesInRoom {
  pub user_id: String,
  pub device_id: String,
  pub room_id: String,
  pub stream_id: i64,
  pub converted_to_destinations: bool,
  pub opentracing_context: Option<String>,
}
