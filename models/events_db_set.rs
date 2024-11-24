#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Events;

pub struct EventsSet;

impl EventsSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events""#)
            .fetch_all(executor)
            .await
    }



    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_token<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_token_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_token_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_account_data_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_account_data_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_account_data_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_as_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_as_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_as_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_txn_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_txn_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_txn_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = $1"#)
            .bind(event_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, event_id_list: Vec<String>) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = ANY($1)"#)
            .bind(event_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = $1"#)
            .bind(event_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_state_key<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_state_key_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_state_key_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_algorithm<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_algorithm_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_algorithm_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_algorithm<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_algorithm_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_algorithm_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_key_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_key_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_key_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = $1"#)
            .bind(event_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, event_id_list: Vec<String>) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = ANY($1)"#)
            .bind(event_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = $1"#)
            .bind(event_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = $1"#)
            .bind(event_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, event_id_list: Vec<String>) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = ANY($1)"#)
            .bind(event_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = $1"#)
            .bind(event_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = $1"#)
            .bind(event_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, event_id_list: Vec<String>) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = ANY($1)"#)
            .bind(event_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = $1"#)
            .bind(event_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = $1"#)
            .bind(event_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, event_id_list: Vec<String>) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = ANY($1)"#)
            .bind(event_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = $1"#)
            .bind(event_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_profile_tag<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_profile_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_profile_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = $1"#)
            .bind(event_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, event_id_list: Vec<String>) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = ANY($1)"#)
            .bind(event_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = $1"#)
            .bind(event_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = $1"#)
            .bind(event_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, event_id_list: Vec<String>) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = ANY($1)"#)
            .bind(event_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = $1"#)
            .bind(event_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_media_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_media_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_media_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = $1"#)
            .bind(event_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, event_id_list: Vec<String>) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = ANY($1)"#)
            .bind(event_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = $1"#)
            .bind(event_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_server_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_server_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_server_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_rule_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_rule_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_rule_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_rule_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_rule_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_rule_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_app_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_app_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_app_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_pushkey<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_pushkey_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_pushkey_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_thread_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_thread_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_thread_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_thread_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_thread_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_thread_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_transaction_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_transaction_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_transaction_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_origin<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_origin_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_origin_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = $1"#)
            .bind(event_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, event_id_list: Vec<String>) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = ANY($1)"#)
            .bind(event_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = $1"#)
            .bind(event_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_token<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_token_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_token_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_token<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_token_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_token_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = $1"#)
            .bind(event_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, event_id_list: Vec<String>) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = ANY($1)"#)
            .bind(event_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = $1"#)
            .bind(event_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_media_origin<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_media_origin_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_media_origin_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_media_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_media_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_media_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_account_data_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_account_data_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_account_data_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_alias<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_alias_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_alias_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = $1"#)
            .bind(event_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, event_id_list: Vec<String>) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = ANY($1)"#)
            .bind(event_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = $1"#)
            .bind(event_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_tag<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_server_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_server_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_server_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_key_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_key_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_key_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_from_server<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_from_server_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_from_server_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_server_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_server_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_server_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_key_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_key_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_key_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = $1"#)
            .bind(event_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, event_id_list: Vec<String>) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = ANY($1)"#)
            .bind(event_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, event_id: String) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "event_id" = $1"#)
            .bind(event_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_stage_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_stage_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_stage_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_ip<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_ip_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_ip_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_agent<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_agent_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_agent_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_auth_provider<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_auth_provider_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_auth_provider_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_external_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_external_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_external_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_address<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_address_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_address_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Events> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Events>> {
        query_as::<_, Events>(r#"SELECT * FROM "events" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, events: Events) -> Result<Events> {
        query_as::<_, Events>(r#"INSERT INTO "events" ("topological_ordering", "event_id", "type", "room_id", "content", "unrecognized_keys", "processed", "outlier", "depth", "origin_server_ts", "received_ts", "sender", "contains_url", "instance_name", "stream_ordering", "state_key", "rejection_reason") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17) RETURNING *;"#)
            .bind(events.topological_ordering)
            .bind(events.event_id)
            .bind(events.type)
            .bind(events.room_id)
            .bind(events.content)
            .bind(events.unrecognized_keys)
            .bind(events.processed)
            .bind(events.outlier)
            .bind(events.depth)
            .bind(events.origin_server_ts)
            .bind(events.received_ts)
            .bind(events.sender)
            .bind(events.contains_url)
            .bind(events.instance_name)
            .bind(events.stream_ordering)
            .bind(events.state_key)
            .bind(events.rejection_reason)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, events: Events) -> Result<Events> {
        query_as::<_, Events>(r#"UPDATE "events" SET "topological_ordering" = $1, "event_id" = $2, "type" = $3, "room_id" = $4, "content" = $5, "unrecognized_keys" = $6, "processed" = $7, "outlier" = $8, "depth" = $9, "origin_server_ts" = $10, "received_ts" = $11, "sender" = $12, "contains_url" = $13, "instance_name" = $14, "stream_ordering" = $15, "state_key" = $16, "rejection_reason" = $17 WHERE  RETURNING *;"#)
            .bind(events.topological_ordering)
            .bind(events.event_id)
            .bind(events.type)
            .bind(events.room_id)
            .bind(events.content)
            .bind(events.unrecognized_keys)
            .bind(events.processed)
            .bind(events.outlier)
            .bind(events.depth)
            .bind(events.origin_server_ts)
            .bind(events.received_ts)
            .bind(events.sender)
            .bind(events.contains_url)
            .bind(events.instance_name)
            .bind(events.stream_ordering)
            .bind(events.state_key)
            .bind(events.rejection_reason)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "events" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
