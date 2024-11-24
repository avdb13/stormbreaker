#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Pushers;

pub struct PushersSet;

impl PushersSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i64) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i64>) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i64) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_token<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_token_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_token_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_account_data_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_account_data_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_account_data_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_as_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_as_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_as_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_txn_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_txn_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_txn_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_state_key<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_state_key_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_state_key_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_algorithm<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_algorithm_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_algorithm_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_algorithm<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_algorithm_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_algorithm_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_key_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_key_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_key_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_profile_tag<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_profile_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_profile_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_media_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_media_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_media_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_server_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_server_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_server_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_name<'e, E: PgExecutor<'e>>(&self, executor: E, user_name: String) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "user_name" = $1"#)
            .bind(user_name)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_name_list: Vec<String>) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "user_name" = ANY($1)"#)
            .bind(user_name_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_name: String) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "user_name" = $1"#)
            .bind(user_name)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_rule_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_rule_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_rule_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_name<'e, E: PgExecutor<'e>>(&self, executor: E, user_name: String) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "user_name" = $1"#)
            .bind(user_name)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_name_list: Vec<String>) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "user_name" = ANY($1)"#)
            .bind(user_name_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_name: String) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "user_name" = $1"#)
            .bind(user_name)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_rule_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_rule_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_rule_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_name<'e, E: PgExecutor<'e>>(&self, executor: E, user_name: String) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "user_name" = $1"#)
            .bind(user_name)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, user_name_list: Vec<String>) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "user_name" = ANY($1)"#)
            .bind(user_name_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, user_name: String) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "user_name" = $1"#)
            .bind(user_name)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_app_id<'e, E: PgExecutor<'e>>(&self, executor: E, app_id: String) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "app_id" = $1"#)
            .bind(app_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_app_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, app_id_list: Vec<String>) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "app_id" = ANY($1)"#)
            .bind(app_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_app_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, app_id: String) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "app_id" = $1"#)
            .bind(app_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_pushkey<'e, E: PgExecutor<'e>>(&self, executor: E, pushkey: String) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "pushkey" = $1"#)
            .bind(pushkey)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_pushkey_list<'e, E: PgExecutor<'e>>(&self, executor: E, pushkey_list: Vec<String>) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "pushkey" = ANY($1)"#)
            .bind(pushkey_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_pushkey_optional<'e, E: PgExecutor<'e>>(&self, executor: E, pushkey: String) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "pushkey" = $1"#)
            .bind(pushkey)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_thread_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_thread_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_thread_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_thread_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_thread_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_thread_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_transaction_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_transaction_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_transaction_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_origin<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_origin_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_origin_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_token<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_token_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_token_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_token<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_token_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_token_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_media_origin<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_media_origin_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_media_origin_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_media_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_media_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_media_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_account_data_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_account_data_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_account_data_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_alias<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_alias_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_alias_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_tag<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_server_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_server_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_server_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_key_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_key_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_key_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_from_server<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_from_server_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_from_server_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_server_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_server_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_server_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_key_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_key_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_key_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_stage_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_stage_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_stage_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_ip<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_ip_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_ip_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_agent<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_agent_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_agent_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_auth_provider<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_auth_provider_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_auth_provider_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_external_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_external_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_external_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_address<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_address_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_address_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Pushers>> {
        query_as::<_, Pushers>(r#"SELECT * FROM "pushers" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, pushers: Pushers) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"INSERT INTO "pushers" ("id", "user_name", "access_token", "profile_tag", "kind", "app_id", "app_display_name", "device_display_name", "pushkey", "ts", "lang", "data", "last_stream_ordering", "last_success", "failing_since") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15) RETURNING *;"#)
            .bind(pushers.id)
            .bind(pushers.user_name)
            .bind(pushers.access_token)
            .bind(pushers.profile_tag)
            .bind(pushers.kind)
            .bind(pushers.app_id)
            .bind(pushers.app_display_name)
            .bind(pushers.device_display_name)
            .bind(pushers.pushkey)
            .bind(pushers.ts)
            .bind(pushers.lang)
            .bind(pushers.data)
            .bind(pushers.last_stream_ordering)
            .bind(pushers.last_success)
            .bind(pushers.failing_since)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, pushers: Pushers) -> Result<Pushers> {
        query_as::<_, Pushers>(r#"UPDATE "pushers" SET "user_name" = $2, "access_token" = $3, "profile_tag" = $4, "kind" = $5, "app_id" = $6, "app_display_name" = $7, "device_display_name" = $8, "pushkey" = $9, "ts" = $10, "lang" = $11, "data" = $12, "last_stream_ordering" = $13, "last_success" = $14, "failing_since" = $15 WHERE "id" = 1 RETURNING *;"#)
            .bind(pushers.id)
            .bind(pushers.user_name)
            .bind(pushers.access_token)
            .bind(pushers.profile_tag)
            .bind(pushers.kind)
            .bind(pushers.app_id)
            .bind(pushers.app_display_name)
            .bind(pushers.device_display_name)
            .bind(pushers.pushkey)
            .bind(pushers.ts)
            .bind(pushers.lang)
            .bind(pushers.data)
            .bind(pushers.last_stream_ordering)
            .bind(pushers.last_success)
            .bind(pushers.failing_since)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "pushers" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
