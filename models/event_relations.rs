#![allow(dead_code)]
pub struct EventRelations {
  pub event_id: String,
  pub relates_to_id: String,
  pub relation_type: String,
  pub aggregation_key: Option<String>,
}
