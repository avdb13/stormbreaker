#![allow(dead_code)]
pub struct PushRulesStream {
  pub stream_id: i64,
  pub event_stream_ordering: i64,
  pub user_id: String,
  pub rule_id: String,
  pub op: String,
  pub priority_class: Option<i16>,
  pub priority: Option<i32>,
  pub conditions: Option<String>,
  pub actions: Option<String>,
}
