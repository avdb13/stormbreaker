#![allow(dead_code)]
pub struct LocalMediaRepositoryUrlCache {
  pub url: Option<String>,
  pub response_code: Option<i32>,
  pub etag: Option<String>,
  pub expires_ts: Option<i64>,
  pub og: Option<String>,
  pub media_id: Option<String>,
  pub download_ts: Option<i64>,
}
