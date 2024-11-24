#![allow(dead_code)]
pub struct ReceivedTransactions {
  pub transaction_id: Option<String>,
  pub origin: Option<String>,
  pub ts: Option<i64>,
  pub response_code: Option<i32>,
  pub response_json: Option<Vec<u8>>,
  pub has_been_referenced: Option<i16>,
}
