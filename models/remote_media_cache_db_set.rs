#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::RemoteMediaCache;

pub struct RemoteMediaCacheSet;

impl RemoteMediaCacheSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache""#)
            .fetch_all(executor)
            .await
    }



    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_token<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_token_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_token_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_account_data_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_account_data_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_account_data_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_as_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_as_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_as_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_txn_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_txn_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_txn_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_state_key<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_state_key_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_state_key_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_algorithm<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_algorithm_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_algorithm_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_device_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_device_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_device_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_algorithm<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_algorithm_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_algorithm_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_key_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_key_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_key_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_profile_tag<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_profile_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_profile_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_media_id<'e, E: PgExecutor<'e>>(&self, executor: E, media_id: String) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "media_id" = $1"#)
            .bind(media_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_media_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, media_id_list: Vec<String>) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "media_id" = ANY($1)"#)
            .bind(media_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_media_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, media_id: String) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "media_id" = $1"#)
            .bind(media_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_server_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_server_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_server_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_rule_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_rule_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_rule_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_rule_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_rule_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_rule_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_app_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_app_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_app_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_pushkey<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_pushkey_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_pushkey_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_thread_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_thread_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_thread_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_receipt_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_receipt_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_receipt_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_thread_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_thread_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_thread_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_transaction_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_transaction_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_transaction_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_origin<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_origin_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_origin_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_token<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_token_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_token_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_token<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_token_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_token_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_media_origin<'e, E: PgExecutor<'e>>(&self, executor: E, media_origin: String) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "media_origin" = $1"#)
            .bind(media_origin)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_media_origin_list<'e, E: PgExecutor<'e>>(&self, executor: E, media_origin_list: Vec<String>) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "media_origin" = ANY($1)"#)
            .bind(media_origin_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_media_origin_optional<'e, E: PgExecutor<'e>>(&self, executor: E, media_origin: String) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "media_origin" = $1"#)
            .bind(media_origin)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_media_id<'e, E: PgExecutor<'e>>(&self, executor: E, media_id: String) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "media_id" = $1"#)
            .bind(media_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_media_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, media_id_list: Vec<String>) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "media_id" = ANY($1)"#)
            .bind(media_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_media_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, media_id: String) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "media_id" = $1"#)
            .bind(media_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_account_data_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_account_data_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_account_data_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_alias<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_alias_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_alias_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_tag<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_room_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_room_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_room_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_server_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_server_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_server_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_key_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_key_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_key_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_from_server<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_from_server_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_from_server_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_server_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_server_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_server_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_key_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_key_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_key_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_stage_type<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_stage_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_stage_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_session_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_session_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_session_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_ip<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_ip_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_ip_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_user_agent<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_user_agent_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_user_agent_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_lock<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_lock_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_lock_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_auth_provider<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_auth_provider_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_auth_provider_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_external_id<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_external_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_external_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_address<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_address_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_address_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RemoteMediaCache>> {
        query_as::<_, RemoteMediaCache>(r#"SELECT * FROM "remote_media_cache" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, remote_media_cache: RemoteMediaCache) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"INSERT INTO "remote_media_cache" ("media_origin", "media_id", "media_type", "created_ts", "upload_name", "media_length", "filesystem_id", "last_access_ts", "quarantined_by") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *;"#)
            .bind(remote_media_cache.media_origin)
            .bind(remote_media_cache.media_id)
            .bind(remote_media_cache.media_type)
            .bind(remote_media_cache.created_ts)
            .bind(remote_media_cache.upload_name)
            .bind(remote_media_cache.media_length)
            .bind(remote_media_cache.filesystem_id)
            .bind(remote_media_cache.last_access_ts)
            .bind(remote_media_cache.quarantined_by)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, remote_media_cache: RemoteMediaCache) -> Result<RemoteMediaCache> {
        query_as::<_, RemoteMediaCache>(r#"UPDATE "remote_media_cache" SET "media_origin" = $1, "media_id" = $2, "media_type" = $3, "created_ts" = $4, "upload_name" = $5, "media_length" = $6, "filesystem_id" = $7, "last_access_ts" = $8, "quarantined_by" = $9 WHERE  RETURNING *;"#)
            .bind(remote_media_cache.media_origin)
            .bind(remote_media_cache.media_id)
            .bind(remote_media_cache.media_type)
            .bind(remote_media_cache.created_ts)
            .bind(remote_media_cache.upload_name)
            .bind(remote_media_cache.media_length)
            .bind(remote_media_cache.filesystem_id)
            .bind(remote_media_cache.last_access_ts)
            .bind(remote_media_cache.quarantined_by)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "remote_media_cache" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
