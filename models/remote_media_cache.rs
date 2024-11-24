#![allow(dead_code)]
pub struct RemoteMediaCache {
  pub media_origin: Option<String>,
  pub media_id: Option<String>,
  pub media_type: Option<String>,
  pub created_ts: Option<i64>,
  pub upload_name: Option<String>,
  pub media_length: Option<i32>,
  pub filesystem_id: Option<String>,
  pub last_access_ts: Option<i64>,
  pub quarantined_by: Option<String>,
}
