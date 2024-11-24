#![allow(dead_code)]
pub struct DeviceListsOutboundPokes {
  pub destination: String,
  pub stream_id: i64,
  pub user_id: String,
  pub device_id: String,
  pub sent: bool,
  pub ts: i64,
  pub opentracing_context: Option<String>,
}
