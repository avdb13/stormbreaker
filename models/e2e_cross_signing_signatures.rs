#![allow(dead_code)]
pub struct E2eCrossSigningSignatures {
  pub user_id: String,
  pub key_id: String,
  pub target_user_id: String,
  pub target_device_id: String,
  pub signature: String,
}
