#![allow(dead_code)]
pub struct Users {
  pub name: Option<String>,
  pub password_hash: Option<String>,
  pub creation_ts: Option<i64>,
  pub admin: i16,
  pub upgrade_ts: Option<i64>,
  pub is_guest: i16,
  pub appservice_id: Option<String>,
  pub consent_version: Option<String>,
  pub consent_server_notice_sent: Option<String>,
  pub user_type: Option<String>,
  pub deactivated: i16,
  pub shadow_banned: Option<bool>,
  pub consent_ts: Option<i64>,
}
