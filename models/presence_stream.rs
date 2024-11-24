#![allow(dead_code)]
pub struct PresenceStream {
  pub stream_id: Option<i64>,
  pub user_id: Option<String>,
  pub state: Option<String>,
  pub last_active_ts: Option<i64>,
  pub last_federation_update_ts: Option<i64>,
  pub last_user_sync_ts: Option<i64>,
  pub status_msg: Option<String>,
  pub currently_active: Option<bool>,
  pub instance_name: Option<String>,
}
