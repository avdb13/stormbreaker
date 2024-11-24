#![allow(dead_code)]
pub struct RoomStatsState {
  pub room_id: String,
  pub name: Option<String>,
  pub canonical_alias: Option<String>,
  pub join_rules: Option<String>,
  pub history_visibility: Option<String>,
  pub encryption: Option<String>,
  pub avatar: Option<String>,
  pub guest_access: Option<String>,
  pub is_federatable: Option<bool>,
  pub topic: Option<String>,
  pub room_type: Option<String>,
}
