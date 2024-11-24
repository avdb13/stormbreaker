#![allow(dead_code)]
pub struct ApplicationServicesState {
  pub as_id: String,
  pub state: Option<String>,
  pub read_receipt_stream_id: Option<i64>,
  pub presence_stream_id: Option<i64>,
  pub to_device_stream_id: Option<i64>,
  pub device_list_stream_id: Option<i64>,
}
