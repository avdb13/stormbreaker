#![allow(dead_code)]
pub struct FederationInboundEventsStaging {
  pub origin: String,
  pub room_id: String,
  pub event_id: String,
  pub received_ts: i64,
  pub event_json: String,
  pub internal_metadata: String,
}
