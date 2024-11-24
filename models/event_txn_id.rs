#![allow(dead_code)]
pub struct EventTxnId {
  pub event_id: String,
  pub room_id: String,
  pub user_id: String,
  pub token_id: i64,
  pub txn_id: String,
  pub inserted_ts: i64,
}
