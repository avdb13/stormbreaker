#![allow(dead_code)]
pub struct Pushers {
  pub id: i64,
  pub user_name: String,
  pub access_token: Option<i64>,
  pub profile_tag: String,
  pub kind: String,
  pub app_id: String,
  pub app_display_name: String,
  pub device_display_name: String,
  pub pushkey: String,
  pub ts: i64,
  pub lang: Option<String>,
  pub data: Option<String>,
  pub last_stream_ordering: Option<i64>,
  pub last_success: Option<i64>,
  pub failing_since: Option<i64>,
}
