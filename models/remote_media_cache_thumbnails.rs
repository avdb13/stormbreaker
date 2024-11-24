#![allow(dead_code)]
pub struct RemoteMediaCacheThumbnails {
  pub media_origin: Option<String>,
  pub media_id: Option<String>,
  pub thumbnail_width: Option<i32>,
  pub thumbnail_height: Option<i32>,
  pub thumbnail_method: Option<String>,
  pub thumbnail_type: Option<String>,
  pub thumbnail_length: Option<i32>,
  pub filesystem_id: Option<String>,
}
