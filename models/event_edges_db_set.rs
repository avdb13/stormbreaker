#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EventEdges;

pub struct EventEdgesSet;

impl EventEdgesSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges""#)
            .fetch_all(executor)
            .await
    }



    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_token<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_token_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_token_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_account_data_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_account_data_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_account_data_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_as_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_as_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_as_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_txn_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_txn_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_txn_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_state_key<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_state_key_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_state_key_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_algorithm<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_algorithm_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_algorithm_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_algorithm<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_algorithm_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_algorithm_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_key_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_key_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_key_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_profile_tag<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_profile_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_profile_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_media_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_media_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_media_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_server_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_server_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_server_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_rule_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_rule_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_rule_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_rule_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_rule_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_rule_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_app_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_app_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_app_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_pushkey<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_pushkey_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_pushkey_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_thread_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_thread_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_thread_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_thread_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_thread_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_thread_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_transaction_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_transaction_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_transaction_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_origin<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_origin_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_origin_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_token<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_token_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_token_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_token<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_token_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_token_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_media_origin<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_media_origin_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_media_origin_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_media_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_media_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_media_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_account_data_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_account_data_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_account_data_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_alias<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_alias_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_alias_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_tag<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_server_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_server_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_server_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_key_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_key_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_key_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_from_server<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_from_server_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_from_server_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_server_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_server_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_server_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_key_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_key_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_key_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_stage_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_stage_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_stage_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_ip<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_ip_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_ip_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_agent<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_agent_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_agent_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_auth_provider<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_auth_provider_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_auth_provider_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_external_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_external_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_external_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_address<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_address_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_address_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_events_event_id<'e, E: PgExecutor<'e>>(executor: E, events_event_id: String) -> Result<Vec<EventEdges>> {
        query_as::<_, EventEdges>(r#"SELECT * FROM "event_edges" WHERE event_id = $1"#)
            .bind(events_event_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, event_edges: EventEdges) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"INSERT INTO "event_edges" ("event_id", "prev_event_id", "room_id", "is_state") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(event_edges.event_id)
            .bind(event_edges.prev_event_id)
            .bind(event_edges.room_id)
            .bind(event_edges.is_state)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, event_edges: EventEdges) -> Result<EventEdges> {
        query_as::<_, EventEdges>(r#"UPDATE "event_edges" SET "event_id" = $1, "prev_event_id" = $2, "room_id" = $3, "is_state" = $4 WHERE  RETURNING *;"#)
            .bind(event_edges.event_id)
            .bind(event_edges.prev_event_id)
            .bind(event_edges.room_id)
            .bind(event_edges.is_state)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "event_edges" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
