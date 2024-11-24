#![allow(dead_code)]
pub struct LocalMediaRepository {
  pub media_id: Option<String>,
  pub media_type: Option<String>,
  pub media_length: Option<i32>,
  pub created_ts: Option<i64>,
  pub upload_name: Option<String>,
  pub user_id: Option<String>,
  pub quarantined_by: Option<String>,
  pub url_cache: Option<String>,
  pub last_access_ts: Option<i64>,
  pub safe_from_quarantine: bool,
}
