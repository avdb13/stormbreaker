#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReceiptsLinearized;

pub struct ReceiptsLinearizedSet;

impl ReceiptsLinearizedSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized""#)
            .fetch_all(executor)
            .await
    }



    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_token<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_token_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_token_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_account_data_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_account_data_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_account_data_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_as_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_as_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_as_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_txn_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_txn_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_txn_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_state_key<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_state_key_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_state_key_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_algorithm<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_algorithm_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_algorithm_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_algorithm<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_algorithm_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_algorithm_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_key_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_key_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_key_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_profile_tag<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_profile_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_profile_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_media_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_media_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_media_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_server_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_server_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_server_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_rule_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_rule_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_rule_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_rule_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_rule_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_rule_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_app_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_app_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_app_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_pushkey<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_pushkey_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_pushkey_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, receipt_type: String, receipt_type: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "receipt_type" = $1 AND "receipt_type" = $2"#)
            .bind(receipt_type)
            .bind(receipt_type)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, receipt_type_list: Vec<String>, receipt_type_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "receipt_type" = ANY($1) AND "receipt_type" = ANY($2)"#)
            .bind(receipt_type_list)
            .bind(receipt_type_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, receipt_type: String, receipt_type: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "receipt_type" = $1 AND "receipt_type" = $2"#)
            .bind(receipt_type)
            .bind(receipt_type)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, receipt_type: String, receipt_type: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "receipt_type" = $1 AND "receipt_type" = $2"#)
            .bind(receipt_type)
            .bind(receipt_type)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, receipt_type_list: Vec<String>, receipt_type_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "receipt_type" = ANY($1) AND "receipt_type" = ANY($2)"#)
            .bind(receipt_type_list)
            .bind(receipt_type_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, receipt_type: String, receipt_type: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "receipt_type" = $1 AND "receipt_type" = $2"#)
            .bind(receipt_type)
            .bind(receipt_type)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_thread_id<'e, E: PgExecutor<'e>>(&self, executor: E, thread_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "thread_id" = $1"#)
            .bind(thread_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_thread_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, thread_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "thread_id" = ANY($1)"#)
            .bind(thread_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_thread_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, thread_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "thread_id" = $1"#)
            .bind(thread_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, receipt_type: String, receipt_type: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "receipt_type" = $1 AND "receipt_type" = $2"#)
            .bind(receipt_type)
            .bind(receipt_type)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, receipt_type_list: Vec<String>, receipt_type_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "receipt_type" = ANY($1) AND "receipt_type" = ANY($2)"#)
            .bind(receipt_type_list)
            .bind(receipt_type_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, receipt_type: String, receipt_type: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "receipt_type" = $1 AND "receipt_type" = $2"#)
            .bind(receipt_type)
            .bind(receipt_type)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, receipt_type: String, receipt_type: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "receipt_type" = $1 AND "receipt_type" = $2"#)
            .bind(receipt_type)
            .bind(receipt_type)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, receipt_type_list: Vec<String>, receipt_type_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "receipt_type" = ANY($1) AND "receipt_type" = ANY($2)"#)
            .bind(receipt_type_list)
            .bind(receipt_type_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, receipt_type: String, receipt_type: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "receipt_type" = $1 AND "receipt_type" = $2"#)
            .bind(receipt_type)
            .bind(receipt_type)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_thread_id<'e, E: PgExecutor<'e>>(&self, executor: E, thread_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "thread_id" = $1"#)
            .bind(thread_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_thread_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, thread_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "thread_id" = ANY($1)"#)
            .bind(thread_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_thread_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, thread_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "thread_id" = $1"#)
            .bind(thread_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_transaction_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_transaction_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_transaction_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_origin<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_origin_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_origin_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_token<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_token_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_token_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_token<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_token_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_token_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_media_origin<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_media_origin_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_media_origin_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_media_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_media_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_media_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_account_data_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_account_data_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_account_data_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_alias<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_alias_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_alias_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_tag<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>, user_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = ANY($1) AND "user_id" = ANY($2)"#)
            .bind(user_id_list)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String, user_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "user_id" = $1 AND "user_id" = $2"#)
            .bind(user_id)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, room_id_list: Vec<String>, room_id_list: Vec<String>) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = ANY($1) AND "room_id" = ANY($2)"#)
            .bind(room_id_list)
            .bind(room_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, room_id: String, room_id: String) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "room_id" = $1 AND "room_id" = $2"#)
            .bind(room_id)
            .bind(room_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_server_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_server_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_server_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_key_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_key_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_key_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_from_server<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_from_server_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_from_server_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_server_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_server_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_server_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_key_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_key_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_key_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_stage_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_stage_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_stage_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_ip<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_ip_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_ip_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_agent<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_agent_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_agent_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_auth_provider<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_auth_provider_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_auth_provider_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_external_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_external_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_external_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_address<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_address_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_address_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReceiptsLinearized>> {
        query_as::<_, ReceiptsLinearized>(r#"SELECT * FROM "receipts_linearized" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, receipts_linearized: ReceiptsLinearized) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"INSERT INTO "receipts_linearized" ("stream_id", "room_id", "room_id", "receipt_type", "receipt_type", "user_id", "user_id", "event_id", "data", "instance_name", "event_stream_ordering", "thread_id") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING *;"#)
            .bind(receipts_linearized.stream_id)
            .bind(receipts_linearized.room_id)
            .bind(receipts_linearized.room_id)
            .bind(receipts_linearized.receipt_type)
            .bind(receipts_linearized.receipt_type)
            .bind(receipts_linearized.user_id)
            .bind(receipts_linearized.user_id)
            .bind(receipts_linearized.event_id)
            .bind(receipts_linearized.data)
            .bind(receipts_linearized.instance_name)
            .bind(receipts_linearized.event_stream_ordering)
            .bind(receipts_linearized.thread_id)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, receipts_linearized: ReceiptsLinearized) -> Result<ReceiptsLinearized> {
        query_as::<_, ReceiptsLinearized>(r#"UPDATE "receipts_linearized" SET "stream_id" = $1, "room_id" = $2, "room_id" = $3, "receipt_type" = $4, "receipt_type" = $5, "user_id" = $6, "user_id" = $7, "event_id" = $8, "data" = $9, "instance_name" = $10, "event_stream_ordering" = $11, "thread_id" = $12 WHERE  RETURNING *;"#)
            .bind(receipts_linearized.stream_id)
            .bind(receipts_linearized.room_id)
            .bind(receipts_linearized.room_id)
            .bind(receipts_linearized.receipt_type)
            .bind(receipts_linearized.receipt_type)
            .bind(receipts_linearized.user_id)
            .bind(receipts_linearized.user_id)
            .bind(receipts_linearized.event_id)
            .bind(receipts_linearized.data)
            .bind(receipts_linearized.instance_name)
            .bind(receipts_linearized.event_stream_ordering)
            .bind(receipts_linearized.thread_id)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "receipts_linearized" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
