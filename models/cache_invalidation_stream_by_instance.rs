#![allow(dead_code)]
pub struct CacheInvalidationStreamByInstance {
  pub stream_id: i64,
  pub instance_name: String,
  pub cache_func: String,
  pub keys: Option<Vec<String>>,
  pub invalidation_ts: Option<i64>,
}
