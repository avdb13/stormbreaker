#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EventLabels;

pub struct EventLabelsSet;

impl EventLabelsSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_and_label<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String, label: String) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "event_id" = $1 AND "label" = $2"#)
            .bind(event_id)
            .bind(label)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_and_label_list<'e, E: PgExecutor<'e>>(&self, executor: E, event_id_list: Vec<String>, label_list: Vec<String>) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "event_id" = ANY($1) AND "label" = ANY($2)"#)
            .bind(event_id_list)
            .bind(label_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_and_label_optional<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String, label: String) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "event_id" = $1 AND "label" = $2"#)
            .bind(event_id)
            .bind(label)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_token<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_token_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_token_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_account_data_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_account_data_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_account_data_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_as_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_as_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_as_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_txn_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_txn_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_txn_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_state_key<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_state_key_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_state_key_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_algorithm<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_algorithm_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_algorithm_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_algorithm<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_algorithm_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_algorithm_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_key_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_key_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_key_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_profile_tag<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_profile_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_profile_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_media_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_media_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_media_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_server_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_server_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_server_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_rule_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_rule_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_rule_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_rule_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_rule_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_rule_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_app_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_app_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_app_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_pushkey<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_pushkey_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_pushkey_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_thread_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_thread_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_thread_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_thread_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_thread_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_thread_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_transaction_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_transaction_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_transaction_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_origin<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_origin_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_origin_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_token<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_token_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_token_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_token<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_token_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_token_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_media_origin<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_media_origin_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_media_origin_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_media_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_media_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_media_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_account_data_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_account_data_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_account_data_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_alias<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_alias_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_alias_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_tag<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_server_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_server_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_server_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_key_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_key_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_key_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_from_server<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_from_server_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_from_server_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_server_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_server_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_server_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_key_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_key_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_key_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_stage_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_stage_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_stage_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_ip<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_ip_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_ip_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_agent<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_agent_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_agent_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_auth_provider<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_auth_provider_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_auth_provider_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_external_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_external_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_external_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_address<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_address_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_address_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventLabels>> {
        query_as::<_, EventLabels>(r#"SELECT * FROM "event_labels" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, event_labels: EventLabels) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"INSERT INTO "event_labels" ("event_id", "label", "room_id", "topological_ordering") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(event_labels.event_id)
            .bind(event_labels.label)
            .bind(event_labels.room_id)
            .bind(event_labels.topological_ordering)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, event_labels: EventLabels) -> Result<EventLabels> {
        query_as::<_, EventLabels>(r#"UPDATE "event_labels" SET "room_id" = $3, "topological_ordering" = $4 WHERE "event_id" = 1 AND "label" = 2 RETURNING *;"#)
            .bind(event_labels.event_id)
            .bind(event_labels.label)
            .bind(event_labels.room_id)
            .bind(event_labels.topological_ordering)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "event_labels" WHERE "event_id" = 1 AND "label" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
