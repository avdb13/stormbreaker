#![allow(dead_code)]
pub struct EventPushActions {
  pub room_id: String,
  pub event_id: String,
  pub user_id: String,
  pub profile_tag: Option<String>,
  pub actions: String,
  pub topological_ordering: Option<i64>,
  pub stream_ordering: Option<i64>,
  pub notif: Option<i16>,
  pub highlight: Option<i16>,
  pub unread: Option<i16>,
  pub thread_id: Option<String>,
}
