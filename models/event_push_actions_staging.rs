#![allow(dead_code)]
pub struct EventPushActionsStaging {
  pub event_id: String,
  pub user_id: String,
  pub actions: String,
  pub notif: i16,
  pub highlight: i16,
  pub unread: Option<i16>,
  pub thread_id: Option<String>,
}
