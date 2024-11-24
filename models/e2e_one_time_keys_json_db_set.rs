#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::E2eOneTimeKeysJson;

pub struct E2eOneTimeKeysJsonSet;

impl E2eOneTimeKeysJsonSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json""#)
            .fetch_all(executor)
            .await
    }



    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_token<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_token_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_token_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = ANY($1)"#)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_account_data_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_account_data_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_account_data_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_as_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_as_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_as_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_txn_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_txn_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_txn_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_state_key<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_state_key_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_state_key_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = ANY($1)"#)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, device_id: String) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "device_id" = $1"#)
            .bind(device_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, device_id_list: Vec<String>) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "device_id" = ANY($1)"#)
            .bind(device_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, device_id: String) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "device_id" = $1"#)
            .bind(device_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = ANY($1)"#)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, device_id: String) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "device_id" = $1"#)
            .bind(device_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, device_id_list: Vec<String>) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "device_id" = ANY($1)"#)
            .bind(device_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, device_id: String) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "device_id" = $1"#)
            .bind(device_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = ANY($1)"#)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, device_id: String) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "device_id" = $1"#)
            .bind(device_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, device_id_list: Vec<String>) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "device_id" = ANY($1)"#)
            .bind(device_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, device_id: String) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "device_id" = $1"#)
            .bind(device_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_algorithm<'e, E: PgExecutor<'e>>(&self, executor: E, algorithm: String) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "algorithm" = $1"#)
            .bind(algorithm)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_algorithm_list<'e, E: PgExecutor<'e>>(&self, executor: E, algorithm_list: Vec<String>) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "algorithm" = ANY($1)"#)
            .bind(algorithm_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_algorithm_optional<'e, E: PgExecutor<'e>>(&self, executor: E, algorithm: String) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "algorithm" = $1"#)
            .bind(algorithm)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = ANY($1)"#)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, device_id: String) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "device_id" = $1"#)
            .bind(device_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, device_id_list: Vec<String>) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "device_id" = ANY($1)"#)
            .bind(device_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, device_id: String) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "device_id" = $1"#)
            .bind(device_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_algorithm<'e, E: PgExecutor<'e>>(&self, executor: E, algorithm: String) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "algorithm" = $1"#)
            .bind(algorithm)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_algorithm_list<'e, E: PgExecutor<'e>>(&self, executor: E, algorithm_list: Vec<String>) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "algorithm" = ANY($1)"#)
            .bind(algorithm_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_algorithm_optional<'e, E: PgExecutor<'e>>(&self, executor: E, algorithm: String) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "algorithm" = $1"#)
            .bind(algorithm)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_key_id<'e, E: PgExecutor<'e>>(&self, executor: E, key_id: String) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "key_id" = $1"#)
            .bind(key_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_key_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, key_id_list: Vec<String>) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "key_id" = ANY($1)"#)
            .bind(key_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_key_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, key_id: String) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "key_id" = $1"#)
            .bind(key_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = ANY($1)"#)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_profile_tag<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_profile_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_profile_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_media_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_media_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_media_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_server_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_server_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_server_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = ANY($1)"#)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = ANY($1)"#)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_rule_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_rule_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_rule_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_rule_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_rule_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_rule_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_app_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_app_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_app_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_pushkey<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_pushkey_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_pushkey_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = ANY($1)"#)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = ANY($1)"#)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_thread_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_thread_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_thread_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = ANY($1)"#)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = ANY($1)"#)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_thread_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_thread_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_thread_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_transaction_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_transaction_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_transaction_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_origin<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_origin_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_origin_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_token<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_token_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_token_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_token<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_token_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_token_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_media_origin<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_media_origin_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_media_origin_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_media_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_media_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_media_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = ANY($1)"#)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_account_data_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_account_data_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_account_data_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_alias<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_alias_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_alias_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = ANY($1)"#)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_tag<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_id_list: Vec<String>) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = ANY($1)"#)
            .bind(user_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_id: String) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_server_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_server_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_server_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_key_id<'e, E: PgExecutor<'e>>(&self, executor: E, key_id: String) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "key_id" = $1"#)
            .bind(key_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_key_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, key_id_list: Vec<String>) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "key_id" = ANY($1)"#)
            .bind(key_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_key_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, key_id: String) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "key_id" = $1"#)
            .bind(key_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_from_server<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_from_server_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_from_server_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_server_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_server_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_server_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_key_id<'e, E: PgExecutor<'e>>(&self, executor: E, key_id: String) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "key_id" = $1"#)
            .bind(key_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_key_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, key_id_list: Vec<String>) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "key_id" = ANY($1)"#)
            .bind(key_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_key_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, key_id: String) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "key_id" = $1"#)
            .bind(key_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_stage_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_stage_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_stage_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_ip<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_ip_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_ip_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_agent<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_agent_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_agent_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_auth_provider<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_auth_provider_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_auth_provider_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_external_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_external_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_external_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_address<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_address_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_address_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<E2eOneTimeKeysJson>> {
        query_as::<_, E2eOneTimeKeysJson>(r#"SELECT * FROM "e2e_one_time_keys_json" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, e2e_one_time_keys_json: E2eOneTimeKeysJson) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"INSERT INTO "e2e_one_time_keys_json" ("user_id", "device_id", "algorithm", "key_id", "ts_added_ms", "key_json") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(e2e_one_time_keys_json.user_id)
            .bind(e2e_one_time_keys_json.device_id)
            .bind(e2e_one_time_keys_json.algorithm)
            .bind(e2e_one_time_keys_json.key_id)
            .bind(e2e_one_time_keys_json.ts_added_ms)
            .bind(e2e_one_time_keys_json.key_json)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, e2e_one_time_keys_json: E2eOneTimeKeysJson) -> Result<E2eOneTimeKeysJson> {
        query_as::<_, E2eOneTimeKeysJson>(r#"UPDATE "e2e_one_time_keys_json" SET "user_id" = $1, "device_id" = $2, "algorithm" = $3, "key_id" = $4, "ts_added_ms" = $5, "key_json" = $6 WHERE  RETURNING *;"#)
            .bind(e2e_one_time_keys_json.user_id)
            .bind(e2e_one_time_keys_json.device_id)
            .bind(e2e_one_time_keys_json.algorithm)
            .bind(e2e_one_time_keys_json.key_id)
            .bind(e2e_one_time_keys_json.ts_added_ms)
            .bind(e2e_one_time_keys_json.key_json)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "e2e_one_time_keys_json" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
