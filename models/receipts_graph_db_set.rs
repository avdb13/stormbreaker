#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReceiptsGraph;

pub struct ReceiptsGraphSet;

impl ReceiptsGraphSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph""#)
            .fetch_all(executor)
            .await
    }



    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_token<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_token_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_token_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_account_data_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_account_data_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_account_data_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_as_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_as_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_as_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_txn_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_txn_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_txn_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_state_key<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_state_key_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_state_key_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_algorithm<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_algorithm_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_algorithm_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_algorithm<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_algorithm_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_algorithm_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_key_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_key_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_key_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_profile_tag<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_profile_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_profile_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_media_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_media_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_media_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_server_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_server_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_server_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_rule_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_rule_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_rule_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_rule_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_rule_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_rule_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_app_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_app_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_app_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_pushkey<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_pushkey_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_pushkey_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, receipt_type: String, receipt_type: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "receipt_type" = $1 AND "receipt_type" = $2"#)
            .bind(receipt_type)
            .bind(receipt_type)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, receipt_type_list: Vec<String>, receipt_type_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "receipt_type" = ANY($1) AND "receipt_type" = ANY($2)"#)
            .bind(receipt_type_list)
            .bind(receipt_type_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, receipt_type: String, receipt_type: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "receipt_type" = $1 AND "receipt_type" = $2"#)
            .bind(receipt_type)
            .bind(receipt_type)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, receipt_type: String, receipt_type: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "receipt_type" = $1 AND "receipt_type" = $2"#)
            .bind(receipt_type)
            .bind(receipt_type)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, receipt_type_list: Vec<String>, receipt_type_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "receipt_type" = ANY($1) AND "receipt_type" = ANY($2)"#)
            .bind(receipt_type_list)
            .bind(receipt_type_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, receipt_type: String, receipt_type: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "receipt_type" = $1 AND "receipt_type" = $2"#)
            .bind(receipt_type)
            .bind(receipt_type)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_thread_id<'e, E: PgExecutor<'e>>(&self, executor: E, thread_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "thread_id" = $1"#)
            .bind(thread_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_thread_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, thread_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "thread_id" = ANY($1)"#)
            .bind(thread_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_thread_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, thread_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "thread_id" = $1"#)
            .bind(thread_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, receipt_type: String, receipt_type: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "receipt_type" = $1 AND "receipt_type" = $2"#)
            .bind(receipt_type)
            .bind(receipt_type)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, receipt_type_list: Vec<String>, receipt_type_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "receipt_type" = ANY($1) AND "receipt_type" = ANY($2)"#)
            .bind(receipt_type_list)
            .bind(receipt_type_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, receipt_type: String, receipt_type: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "receipt_type" = $1 AND "receipt_type" = $2"#)
            .bind(receipt_type)
            .bind(receipt_type)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, receipt_type: String, receipt_type: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "receipt_type" = $1 AND "receipt_type" = $2"#)
            .bind(receipt_type)
            .bind(receipt_type)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, receipt_type_list: Vec<String>, receipt_type_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "receipt_type" = ANY($1) AND "receipt_type" = ANY($2)"#)
            .bind(receipt_type_list)
            .bind(receipt_type_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, receipt_type: String, receipt_type: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "receipt_type" = $1 AND "receipt_type" = $2"#)
            .bind(receipt_type)
            .bind(receipt_type)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_thread_id<'e, E: PgExecutor<'e>>(&self, executor: E, thread_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "thread_id" = $1"#)
            .bind(thread_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_thread_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, thread_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "thread_id" = ANY($1)"#)
            .bind(thread_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_thread_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, thread_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "thread_id" = $1"#)
            .bind(thread_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_transaction_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_transaction_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_transaction_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_origin<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_origin_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_origin_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_token<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_token_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_token_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_token<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_token_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_token_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_media_origin<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_media_origin_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_media_origin_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_media_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_media_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_media_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_account_data_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_account_data_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_account_data_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_alias<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_alias_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_alias_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_tag<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_server_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_server_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_server_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_key_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_key_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_key_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_from_server<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_from_server_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_from_server_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_server_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_server_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_server_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_key_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_key_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_key_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_stage_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_stage_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_stage_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_ip<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_ip_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_ip_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_agent<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_agent_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_agent_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_auth_provider<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_auth_provider_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_auth_provider_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_external_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_external_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_external_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_address<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_address_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_address_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsGraph>> {
        query_as::<_, ReceiptsGraph>(r#"SELECT * FROM "receipts_graph" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, receipts_graph: ReceiptsGraph) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"INSERT INTO "receipts_graph" ("room_id", "room_id", "receipt_type", "receipt_type", "user_id", "user_id", "event_ids", "data", "thread_id") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *;"#)
            .bind(receipts_graph.room_id)
            .bind(receipts_graph.room_id)
            .bind(receipts_graph.receipt_type)
            .bind(receipts_graph.receipt_type)
            .bind(receipts_graph.user_id)
            .bind(receipts_graph.user_id)
            .bind(receipts_graph.event_ids)
            .bind(receipts_graph.data)
            .bind(receipts_graph.thread_id)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, receipts_graph: ReceiptsGraph) -> Result<ReceiptsGraph> {
        query_as::<_, ReceiptsGraph>(r#"UPDATE "receipts_graph" SET "room_id" = $1, "room_id" = $2, "receipt_type" = $3, "receipt_type" = $4, "user_id" = $5, "user_id" = $6, "event_ids" = $7, "data" = $8, "thread_id" = $9 WHERE  RETURNING *;"#)
            .bind(receipts_graph.room_id)
            .bind(receipts_graph.room_id)
            .bind(receipts_graph.receipt_type)
            .bind(receipts_graph.receipt_type)
            .bind(receipts_graph.user_id)
            .bind(receipts_graph.user_id)
            .bind(receipts_graph.event_ids)
            .bind(receipts_graph.data)
            .bind(receipts_graph.thread_id)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "receipts_graph" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
