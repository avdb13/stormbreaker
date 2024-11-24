#![allow(dead_code)]
// Generated with sql-gen
//https://github.com/jayy-lmao/sql-gen

pub mod access_tokens;
pub use access_tokens::AccessTokens;
pub mod access_tokens_db_set;
pub use access_tokens_db_set::AccessTokensSet;

pub mod account_data;
pub use account_data::AccountData;
pub mod account_data_db_set;
pub use account_data_db_set::AccountDataSet;

pub mod account_validity;
pub use account_validity::AccountValidity;
pub mod account_validity_db_set;
pub use account_validity_db_set::AccountValiditySet;

pub mod application_services_state;
pub use application_services_state::ApplicationServicesState;
pub mod application_services_state_db_set;
pub use application_services_state_db_set::ApplicationServicesStateSet;

pub mod application_services_txns;
pub use application_services_txns::ApplicationServicesTxns;
pub mod application_services_txns_db_set;
pub use application_services_txns_db_set::ApplicationServicesTxnsSet;

pub mod appservice_room_list;
pub use appservice_room_list::AppserviceRoomList;
pub mod appservice_room_list_db_set;
pub use appservice_room_list_db_set::AppserviceRoomListSet;

pub mod appservice_stream_position;
pub use appservice_stream_position::AppserviceStreamPosition;
pub mod appservice_stream_position_db_set;
pub use appservice_stream_position_db_set::AppserviceStreamPositionSet;

pub mod batch_events;
pub use batch_events::BatchEvents;
pub mod batch_events_db_set;
pub use batch_events_db_set::BatchEventsSet;

pub mod blocked_rooms;
pub use blocked_rooms::BlockedRooms;
pub mod blocked_rooms_db_set;
pub use blocked_rooms_db_set::BlockedRoomsSet;

pub mod cache_invalidation_stream_by_instance;
pub use cache_invalidation_stream_by_instance::CacheInvalidationStreamByInstance;
pub mod cache_invalidation_stream_by_instance_db_set;
pub use cache_invalidation_stream_by_instance_db_set::CacheInvalidationStreamByInstanceSet;

pub mod current_state_delta_stream;
pub use current_state_delta_stream::CurrentStateDeltaStream;
pub mod current_state_delta_stream_db_set;
pub use current_state_delta_stream_db_set::CurrentStateDeltaStreamSet;

pub mod current_state_events;
pub use current_state_events::CurrentStateEvents;
pub mod current_state_events_db_set;
pub use current_state_events_db_set::CurrentStateEventsSet;

pub mod dehydrated_devices;
pub use dehydrated_devices::DehydratedDevices;
pub mod dehydrated_devices_db_set;
pub use dehydrated_devices_db_set::DehydratedDevicesSet;

pub mod deleted_pushers;
pub use deleted_pushers::DeletedPushers;
pub mod deleted_pushers_db_set;
pub use deleted_pushers_db_set::DeletedPushersSet;

pub mod destination_rooms;
pub use destination_rooms::DestinationRooms;
pub mod destination_rooms_db_set;
pub use destination_rooms_db_set::DestinationRoomsSet;

pub mod destinations;
pub use destinations::Destinations;
pub mod destinations_db_set;
pub use destinations_db_set::DestinationsSet;

pub mod device_auth_providers;
pub use device_auth_providers::DeviceAuthProviders;
pub mod device_auth_providers_db_set;
pub use device_auth_providers_db_set::DeviceAuthProvidersSet;

pub mod device_federation_inbox;
pub use device_federation_inbox::DeviceFederationInbox;
pub mod device_federation_inbox_db_set;
pub use device_federation_inbox_db_set::DeviceFederationInboxSet;

pub mod device_federation_outbox;
pub use device_federation_outbox::DeviceFederationOutbox;
pub mod device_federation_outbox_db_set;
pub use device_federation_outbox_db_set::DeviceFederationOutboxSet;

pub mod device_inbox;
pub use device_inbox::DeviceInbox;
pub mod device_inbox_db_set;
pub use device_inbox_db_set::DeviceInboxSet;

pub mod device_lists_changes_in_room;
pub use device_lists_changes_in_room::DeviceListsChangesInRoom;
pub mod device_lists_changes_in_room_db_set;
pub use device_lists_changes_in_room_db_set::DeviceListsChangesInRoomSet;

pub mod device_lists_outbound_last_success;
pub use device_lists_outbound_last_success::DeviceListsOutboundLastSuccess;
pub mod device_lists_outbound_last_success_db_set;
pub use device_lists_outbound_last_success_db_set::DeviceListsOutboundLastSuccessSet;

pub mod device_lists_outbound_pokes;
pub use device_lists_outbound_pokes::DeviceListsOutboundPokes;
pub mod device_lists_outbound_pokes_db_set;
pub use device_lists_outbound_pokes_db_set::DeviceListsOutboundPokesSet;

pub mod device_lists_remote_cache;
pub use device_lists_remote_cache::DeviceListsRemoteCache;
pub mod device_lists_remote_cache_db_set;
pub use device_lists_remote_cache_db_set::DeviceListsRemoteCacheSet;

pub mod device_lists_remote_extremeties;
pub use device_lists_remote_extremeties::DeviceListsRemoteExtremeties;
pub mod device_lists_remote_extremeties_db_set;
pub use device_lists_remote_extremeties_db_set::DeviceListsRemoteExtremetiesSet;

pub mod device_lists_remote_resync;
pub use device_lists_remote_resync::DeviceListsRemoteResync;
pub mod device_lists_remote_resync_db_set;
pub use device_lists_remote_resync_db_set::DeviceListsRemoteResyncSet;

pub mod device_lists_stream;
pub use device_lists_stream::DeviceListsStream;
pub mod device_lists_stream_db_set;
pub use device_lists_stream_db_set::DeviceListsStreamSet;

pub mod devices;
pub use devices::Devices;
pub mod devices_db_set;
pub use devices_db_set::DevicesSet;

pub mod e2e_cross_signing_keys;
pub use e2e_cross_signing_keys::E2eCrossSigningKeys;
pub mod e2e_cross_signing_keys_db_set;
pub use e2e_cross_signing_keys_db_set::E2eCrossSigningKeysSet;

pub mod e2e_cross_signing_signatures;
pub use e2e_cross_signing_signatures::E2eCrossSigningSignatures;
pub mod e2e_cross_signing_signatures_db_set;
pub use e2e_cross_signing_signatures_db_set::E2eCrossSigningSignaturesSet;

pub mod e2e_device_keys_json;
pub use e2e_device_keys_json::E2eDeviceKeysJson;
pub mod e2e_device_keys_json_db_set;
pub use e2e_device_keys_json_db_set::E2eDeviceKeysJsonSet;

pub mod e2e_fallback_keys_json;
pub use e2e_fallback_keys_json::E2eFallbackKeysJson;
pub mod e2e_fallback_keys_json_db_set;
pub use e2e_fallback_keys_json_db_set::E2eFallbackKeysJsonSet;

pub mod e2e_one_time_keys_json;
pub use e2e_one_time_keys_json::E2eOneTimeKeysJson;
pub mod e2e_one_time_keys_json_db_set;
pub use e2e_one_time_keys_json_db_set::E2eOneTimeKeysJsonSet;

pub mod e2e_room_keys;
pub use e2e_room_keys::E2eRoomKeys;
pub mod e2e_room_keys_db_set;
pub use e2e_room_keys_db_set::E2eRoomKeysSet;

pub mod e2e_room_keys_versions;
pub use e2e_room_keys_versions::E2eRoomKeysVersions;
pub mod e2e_room_keys_versions_db_set;
pub use e2e_room_keys_versions_db_set::E2eRoomKeysVersionsSet;

pub mod erased_users;
pub use erased_users::ErasedUsers;
pub mod erased_users_db_set;
pub use erased_users_db_set::ErasedUsersSet;

pub mod event_auth;
pub use event_auth::EventAuth;
pub mod event_auth_db_set;
pub use event_auth_db_set::EventAuthSet;

pub mod event_auth_chain_links;
pub use event_auth_chain_links::EventAuthChainLinks;
pub mod event_auth_chain_links_db_set;
pub use event_auth_chain_links_db_set::EventAuthChainLinksSet;

pub mod event_auth_chain_to_calculate;
pub use event_auth_chain_to_calculate::EventAuthChainToCalculate;
pub mod event_auth_chain_to_calculate_db_set;
pub use event_auth_chain_to_calculate_db_set::EventAuthChainToCalculateSet;

pub mod event_auth_chains;
pub use event_auth_chains::EventAuthChains;
pub mod event_auth_chains_db_set;
pub use event_auth_chains_db_set::EventAuthChainsSet;

pub mod event_backward_extremities;
pub use event_backward_extremities::EventBackwardExtremities;
pub mod event_backward_extremities_db_set;
pub use event_backward_extremities_db_set::EventBackwardExtremitiesSet;

pub mod event_edges;
pub use event_edges::EventEdges;
pub mod event_edges_db_set;
pub use event_edges_db_set::EventEdgesSet;

pub mod event_expiry;
pub use event_expiry::EventExpiry;
pub mod event_expiry_db_set;
pub use event_expiry_db_set::EventExpirySet;

pub mod event_forward_extremities;
pub use event_forward_extremities::EventForwardExtremities;
pub mod event_forward_extremities_db_set;
pub use event_forward_extremities_db_set::EventForwardExtremitiesSet;

pub mod event_json;
pub use event_json::EventJson;
pub mod event_json_db_set;
pub use event_json_db_set::EventJsonSet;

pub mod event_labels;
pub use event_labels::EventLabels;
pub mod event_labels_db_set;
pub use event_labels_db_set::EventLabelsSet;

pub mod event_push_actions;
pub use event_push_actions::EventPushActions;
pub mod event_push_actions_db_set;
pub use event_push_actions_db_set::EventPushActionsSet;

pub mod event_push_actions_staging;
pub use event_push_actions_staging::EventPushActionsStaging;
pub mod event_push_actions_staging_db_set;
pub use event_push_actions_staging_db_set::EventPushActionsStagingSet;

pub mod event_push_summary;
pub use event_push_summary::EventPushSummary;
pub mod event_push_summary_db_set;
pub use event_push_summary_db_set::EventPushSummarySet;

pub mod event_push_summary_last_receipt_stream_id;
pub use event_push_summary_last_receipt_stream_id::EventPushSummaryLastReceiptStreamId;
pub mod event_push_summary_last_receipt_stream_id_db_set;
pub use event_push_summary_last_receipt_stream_id_db_set::EventPushSummaryLastReceiptStreamIdSet;

pub mod event_push_summary_stream_ordering;
pub use event_push_summary_stream_ordering::EventPushSummaryStreamOrdering;
pub mod event_push_summary_stream_ordering_db_set;
pub use event_push_summary_stream_ordering_db_set::EventPushSummaryStreamOrderingSet;

pub mod event_relations;
pub use event_relations::EventRelations;
pub mod event_relations_db_set;
pub use event_relations_db_set::EventRelationsSet;

pub mod event_reports;
pub use event_reports::EventReports;
pub mod event_reports_db_set;
pub use event_reports_db_set::EventReportsSet;

pub mod event_search;
pub use event_search::EventSearch;
pub mod event_search_db_set;
pub use event_search_db_set::EventSearchSet;

pub mod event_to_state_groups;
pub use event_to_state_groups::EventToStateGroups;
pub mod event_to_state_groups_db_set;
pub use event_to_state_groups_db_set::EventToStateGroupsSet;

pub mod event_txn_id;
pub use event_txn_id::EventTxnId;
pub mod event_txn_id_db_set;
pub use event_txn_id_db_set::EventTxnIdSet;

pub mod events;
pub use events::Events;
pub mod events_db_set;
pub use events_db_set::EventsSet;

pub mod ex_outlier_stream;
pub use ex_outlier_stream::ExOutlierStream;
pub mod ex_outlier_stream_db_set;
pub use ex_outlier_stream_db_set::ExOutlierStreamSet;

pub mod federation_inbound_events_staging;
pub use federation_inbound_events_staging::FederationInboundEventsStaging;
pub mod federation_inbound_events_staging_db_set;
pub use federation_inbound_events_staging_db_set::FederationInboundEventsStagingSet;

pub mod federation_stream_position;
pub use federation_stream_position::FederationStreamPosition;
pub mod federation_stream_position_db_set;
pub use federation_stream_position_db_set::FederationStreamPositionSet;

pub mod ignored_users;
pub use ignored_users::IgnoredUsers;
pub mod ignored_users_db_set;
pub use ignored_users_db_set::IgnoredUsersSet;

pub mod insertion_event_edges;
pub use insertion_event_edges::InsertionEventEdges;
pub mod insertion_event_edges_db_set;
pub use insertion_event_edges_db_set::InsertionEventEdgesSet;

pub mod insertion_event_extremities;
pub use insertion_event_extremities::InsertionEventExtremities;
pub mod insertion_event_extremities_db_set;
pub use insertion_event_extremities_db_set::InsertionEventExtremitiesSet;

pub mod insertion_events;
pub use insertion_events::InsertionEvents;
pub mod insertion_events_db_set;
pub use insertion_events_db_set::InsertionEventsSet;

pub mod instance_map;
pub use instance_map::InstanceMap;
pub mod instance_map_db_set;
pub use instance_map_db_set::InstanceMapSet;

pub mod local_current_membership;
pub use local_current_membership::LocalCurrentMembership;
pub mod local_current_membership_db_set;
pub use local_current_membership_db_set::LocalCurrentMembershipSet;

pub mod local_media_repository;
pub use local_media_repository::LocalMediaRepository;
pub mod local_media_repository_db_set;
pub use local_media_repository_db_set::LocalMediaRepositorySet;

pub mod local_media_repository_thumbnails;
pub use local_media_repository_thumbnails::LocalMediaRepositoryThumbnails;
pub mod local_media_repository_thumbnails_db_set;
pub use local_media_repository_thumbnails_db_set::LocalMediaRepositoryThumbnailsSet;

pub mod local_media_repository_url_cache;
pub use local_media_repository_url_cache::LocalMediaRepositoryUrlCache;
pub mod local_media_repository_url_cache_db_set;
pub use local_media_repository_url_cache_db_set::LocalMediaRepositoryUrlCacheSet;

pub mod monthly_active_users;
pub use monthly_active_users::MonthlyActiveUsers;
pub mod monthly_active_users_db_set;
pub use monthly_active_users_db_set::MonthlyActiveUsersSet;

pub mod open_id_tokens;
pub use open_id_tokens::OpenIdTokens;
pub mod open_id_tokens_db_set;
pub use open_id_tokens_db_set::OpenIdTokensSet;

pub mod partial_state_events;
pub use partial_state_events::PartialStateEvents;
pub mod partial_state_events_db_set;
pub use partial_state_events_db_set::PartialStateEventsSet;

pub mod partial_state_rooms;
pub use partial_state_rooms::PartialStateRooms;
pub mod partial_state_rooms_db_set;
pub use partial_state_rooms_db_set::PartialStateRoomsSet;

pub mod partial_state_rooms_servers;
pub use partial_state_rooms_servers::PartialStateRoomsServers;
pub mod partial_state_rooms_servers_db_set;
pub use partial_state_rooms_servers_db_set::PartialStateRoomsServersSet;

pub mod presence;
pub use presence::Presence;
pub mod presence_db_set;
pub use presence_db_set::PresenceSet;

pub mod presence_stream;
pub use presence_stream::PresenceStream;
pub mod presence_stream_db_set;
pub use presence_stream_db_set::PresenceStreamSet;

pub mod profiles;
pub use profiles::Profiles;
pub mod profiles_db_set;
pub use profiles_db_set::ProfilesSet;

pub mod push_rules;
pub use push_rules::PushRules;
pub mod push_rules_db_set;
pub use push_rules_db_set::PushRulesSet;

pub mod push_rules_enable;
pub use push_rules_enable::PushRulesEnable;
pub mod push_rules_enable_db_set;
pub use push_rules_enable_db_set::PushRulesEnableSet;

pub mod push_rules_stream;
pub use push_rules_stream::PushRulesStream;
pub mod push_rules_stream_db_set;
pub use push_rules_stream_db_set::PushRulesStreamSet;

pub mod pusher_throttle;
pub use pusher_throttle::PusherThrottle;
pub mod pusher_throttle_db_set;
pub use pusher_throttle_db_set::PusherThrottleSet;

pub mod pushers;
pub use pushers::Pushers;
pub mod pushers_db_set;
pub use pushers_db_set::PushersSet;

pub mod ratelimit_override;
pub use ratelimit_override::RatelimitOverride;
pub mod ratelimit_override_db_set;
pub use ratelimit_override_db_set::RatelimitOverrideSet;

pub mod receipts_graph;
pub use receipts_graph::ReceiptsGraph;
pub mod receipts_graph_db_set;
pub use receipts_graph_db_set::ReceiptsGraphSet;

pub mod receipts_linearized;
pub use receipts_linearized::ReceiptsLinearized;
pub mod receipts_linearized_db_set;
pub use receipts_linearized_db_set::ReceiptsLinearizedSet;

pub mod received_transactions;
pub use received_transactions::ReceivedTransactions;
pub mod received_transactions_db_set;
pub use received_transactions_db_set::ReceivedTransactionsSet;

pub mod redactions;
pub use redactions::Redactions;
pub mod redactions_db_set;
pub use redactions_db_set::RedactionsSet;

pub mod refresh_tokens;
pub use refresh_tokens::RefreshTokens;
pub mod refresh_tokens_db_set;
pub use refresh_tokens_db_set::RefreshTokensSet;

pub mod registration_tokens;
pub use registration_tokens::RegistrationTokens;
pub mod registration_tokens_db_set;
pub use registration_tokens_db_set::RegistrationTokensSet;

pub mod rejections;
pub use rejections::Rejections;
pub mod rejections_db_set;
pub use rejections_db_set::RejectionsSet;

pub mod remote_media_cache;
pub use remote_media_cache::RemoteMediaCache;
pub mod remote_media_cache_db_set;
pub use remote_media_cache_db_set::RemoteMediaCacheSet;

pub mod remote_media_cache_thumbnails;
pub use remote_media_cache_thumbnails::RemoteMediaCacheThumbnails;
pub mod remote_media_cache_thumbnails_db_set;
pub use remote_media_cache_thumbnails_db_set::RemoteMediaCacheThumbnailsSet;

pub mod room_account_data;
pub use room_account_data::RoomAccountData;
pub mod room_account_data_db_set;
pub use room_account_data_db_set::RoomAccountDataSet;

pub mod room_alias_servers;
pub use room_alias_servers::RoomAliasServers;
pub mod room_alias_servers_db_set;
pub use room_alias_servers_db_set::RoomAliasServersSet;

pub mod room_aliases;
pub use room_aliases::RoomAliases;
pub mod room_aliases_db_set;
pub use room_aliases_db_set::RoomAliasesSet;

pub mod room_depth;
pub use room_depth::RoomDepth;
pub mod room_depth_db_set;
pub use room_depth_db_set::RoomDepthSet;

pub mod room_memberships;
pub use room_memberships::RoomMemberships;
pub mod room_memberships_db_set;
pub use room_memberships_db_set::RoomMembershipsSet;

pub mod room_retention;
pub use room_retention::RoomRetention;
pub mod room_retention_db_set;
pub use room_retention_db_set::RoomRetentionSet;

pub mod room_stats_current;
pub use room_stats_current::RoomStatsCurrent;
pub mod room_stats_current_db_set;
pub use room_stats_current_db_set::RoomStatsCurrentSet;

pub mod room_stats_earliest_token;
pub use room_stats_earliest_token::RoomStatsEarliestToken;
pub mod room_stats_earliest_token_db_set;
pub use room_stats_earliest_token_db_set::RoomStatsEarliestTokenSet;

pub mod room_stats_state;
pub use room_stats_state::RoomStatsState;
pub mod room_stats_state_db_set;
pub use room_stats_state_db_set::RoomStatsStateSet;

pub mod room_tags;
pub use room_tags::RoomTags;
pub mod room_tags_db_set;
pub use room_tags_db_set::RoomTagsSet;

pub mod room_tags_revisions;
pub use room_tags_revisions::RoomTagsRevisions;
pub mod room_tags_revisions_db_set;
pub use room_tags_revisions_db_set::RoomTagsRevisionsSet;

pub mod rooms;
pub use rooms::Rooms;
pub mod rooms_db_set;
pub use rooms_db_set::RoomsSet;

pub mod server_keys_json;
pub use server_keys_json::ServerKeysJson;
pub mod server_keys_json_db_set;
pub use server_keys_json_db_set::ServerKeysJsonSet;

pub mod server_signature_keys;
pub use server_signature_keys::ServerSignatureKeys;
pub mod server_signature_keys_db_set;
pub use server_signature_keys_db_set::ServerSignatureKeysSet;

pub mod sessions;
pub use sessions::Sessions;
pub mod sessions_db_set;
pub use sessions_db_set::SessionsSet;

pub mod state_events;
pub use state_events::StateEvents;
pub mod state_events_db_set;
pub use state_events_db_set::StateEventsSet;

pub mod stats_incremental_position;
pub use stats_incremental_position::StatsIncrementalPosition;
pub mod stats_incremental_position_db_set;
pub use stats_incremental_position_db_set::StatsIncrementalPositionSet;

pub mod stream_ordering_to_exterm;
pub use stream_ordering_to_exterm::StreamOrderingToExterm;
pub mod stream_ordering_to_exterm_db_set;
pub use stream_ordering_to_exterm_db_set::StreamOrderingToExtermSet;

pub mod stream_positions;
pub use stream_positions::StreamPositions;
pub mod stream_positions_db_set;
pub use stream_positions_db_set::StreamPositionsSet;

pub mod threepid_guest_access_tokens;
pub use threepid_guest_access_tokens::ThreepidGuestAccessTokens;
pub mod threepid_guest_access_tokens_db_set;
pub use threepid_guest_access_tokens_db_set::ThreepidGuestAccessTokensSet;

pub mod threepid_validation_session;
pub use threepid_validation_session::ThreepidValidationSession;
pub mod threepid_validation_session_db_set;
pub use threepid_validation_session_db_set::ThreepidValidationSessionSet;

pub mod threepid_validation_token;
pub use threepid_validation_token::ThreepidValidationToken;
pub mod threepid_validation_token_db_set;
pub use threepid_validation_token_db_set::ThreepidValidationTokenSet;

pub mod ui_auth_sessions;
pub use ui_auth_sessions::UiAuthSessions;
pub mod ui_auth_sessions_db_set;
pub use ui_auth_sessions_db_set::UiAuthSessionsSet;

pub mod ui_auth_sessions_credentials;
pub use ui_auth_sessions_credentials::UiAuthSessionsCredentials;
pub mod ui_auth_sessions_credentials_db_set;
pub use ui_auth_sessions_credentials_db_set::UiAuthSessionsCredentialsSet;

pub mod ui_auth_sessions_ips;
pub use ui_auth_sessions_ips::UiAuthSessionsIps;
pub mod ui_auth_sessions_ips_db_set;
pub use ui_auth_sessions_ips_db_set::UiAuthSessionsIpsSet;

pub mod user_daily_visits;
pub use user_daily_visits::UserDailyVisits;
pub mod user_daily_visits_db_set;
pub use user_daily_visits_db_set::UserDailyVisitsSet;

pub mod user_directory;
pub use user_directory::UserDirectory;
pub mod user_directory_db_set;
pub use user_directory_db_set::UserDirectorySet;

pub mod user_directory_search;
pub use user_directory_search::UserDirectorySearch;
pub mod user_directory_search_db_set;
pub use user_directory_search_db_set::UserDirectorySearchSet;

pub mod user_directory_stream_pos;
pub use user_directory_stream_pos::UserDirectoryStreamPos;
pub mod user_directory_stream_pos_db_set;
pub use user_directory_stream_pos_db_set::UserDirectoryStreamPosSet;

pub mod user_external_ids;
pub use user_external_ids::UserExternalIds;
pub mod user_external_ids_db_set;
pub use user_external_ids_db_set::UserExternalIdsSet;

pub mod user_filters;
pub use user_filters::UserFilters;
pub mod user_filters_db_set;
pub use user_filters_db_set::UserFiltersSet;

pub mod user_ips;
pub use user_ips::UserIps;
pub mod user_ips_db_set;
pub use user_ips_db_set::UserIpsSet;

pub mod user_signature_stream;
pub use user_signature_stream::UserSignatureStream;
pub mod user_signature_stream_db_set;
pub use user_signature_stream_db_set::UserSignatureStreamSet;

pub mod user_stats_current;
pub use user_stats_current::UserStatsCurrent;
pub mod user_stats_current_db_set;
pub use user_stats_current_db_set::UserStatsCurrentSet;

pub mod user_threepid_id_server;
pub use user_threepid_id_server::UserThreepidIdServer;
pub mod user_threepid_id_server_db_set;
pub use user_threepid_id_server_db_set::UserThreepidIdServerSet;

pub mod user_threepids;
pub use user_threepids::UserThreepids;
pub mod user_threepids_db_set;
pub use user_threepids_db_set::UserThreepidsSet;

pub mod users;
pub use users::Users;
pub mod users_db_set;
pub use users_db_set::UsersSet;

pub mod users_in_public_rooms;
pub use users_in_public_rooms::UsersInPublicRooms;
pub mod users_in_public_rooms_db_set;
pub use users_in_public_rooms_db_set::UsersInPublicRoomsSet;

pub mod users_pending_deactivation;
pub use users_pending_deactivation::UsersPendingDeactivation;
pub mod users_pending_deactivation_db_set;
pub use users_pending_deactivation_db_set::UsersPendingDeactivationSet;

pub mod users_to_send_full_presence_to;
pub use users_to_send_full_presence_to::UsersToSendFullPresenceTo;
pub mod users_to_send_full_presence_to_db_set;
pub use users_to_send_full_presence_to_db_set::UsersToSendFullPresenceToSet;

pub mod users_who_share_private_rooms;
pub use users_who_share_private_rooms::UsersWhoSharePrivateRooms;
pub mod users_who_share_private_rooms_db_set;
pub use users_who_share_private_rooms_db_set::UsersWhoSharePrivateRoomsSet;

pub mod worker_locks;
pub use worker_locks::WorkerLocks;
pub mod worker_locks_db_set;
pub use worker_locks_db_set::WorkerLocksSet;


pub struct AtprotoContext;

impl AtprotoContext {
  pub fn access_tokens(&self) -> AccessTokensSet { AccessTokensSet }

  pub fn account_data(&self) -> AccountDataSet { AccountDataSet }

  pub fn account_validity(&self) -> AccountValiditySet { AccountValiditySet }

  pub fn application_services_state(&self) -> ApplicationServicesStateSet { ApplicationServicesStateSet }

  pub fn application_services_txns(&self) -> ApplicationServicesTxnsSet { ApplicationServicesTxnsSet }

  pub fn appservice_room_list(&self) -> AppserviceRoomListSet { AppserviceRoomListSet }

  pub fn appservice_stream_position(&self) -> AppserviceStreamPositionSet { AppserviceStreamPositionSet }

  pub fn batch_events(&self) -> BatchEventsSet { BatchEventsSet }

  pub fn blocked_rooms(&self) -> BlockedRoomsSet { BlockedRoomsSet }

  pub fn cache_invalidation_stream_by_instance(&self) -> CacheInvalidationStreamByInstanceSet { CacheInvalidationStreamByInstanceSet }

  pub fn current_state_delta_stream(&self) -> CurrentStateDeltaStreamSet { CurrentStateDeltaStreamSet }

  pub fn current_state_events(&self) -> CurrentStateEventsSet { CurrentStateEventsSet }

  pub fn dehydrated_devices(&self) -> DehydratedDevicesSet { DehydratedDevicesSet }

  pub fn deleted_pushers(&self) -> DeletedPushersSet { DeletedPushersSet }

  pub fn destination_rooms(&self) -> DestinationRoomsSet { DestinationRoomsSet }

  pub fn destinations(&self) -> DestinationsSet { DestinationsSet }

  pub fn device_auth_providers(&self) -> DeviceAuthProvidersSet { DeviceAuthProvidersSet }

  pub fn device_federation_inbox(&self) -> DeviceFederationInboxSet { DeviceFederationInboxSet }

  pub fn device_federation_outbox(&self) -> DeviceFederationOutboxSet { DeviceFederationOutboxSet }

  pub fn device_inbox(&self) -> DeviceInboxSet { DeviceInboxSet }

  pub fn device_lists_changes_in_room(&self) -> DeviceListsChangesInRoomSet { DeviceListsChangesInRoomSet }

  pub fn device_lists_outbound_last_success(&self) -> DeviceListsOutboundLastSuccessSet { DeviceListsOutboundLastSuccessSet }

  pub fn device_lists_outbound_pokes(&self) -> DeviceListsOutboundPokesSet { DeviceListsOutboundPokesSet }

  pub fn device_lists_remote_cache(&self) -> DeviceListsRemoteCacheSet { DeviceListsRemoteCacheSet }

  pub fn device_lists_remote_extremeties(&self) -> DeviceListsRemoteExtremetiesSet { DeviceListsRemoteExtremetiesSet }

  pub fn device_lists_remote_resync(&self) -> DeviceListsRemoteResyncSet { DeviceListsRemoteResyncSet }

  pub fn device_lists_stream(&self) -> DeviceListsStreamSet { DeviceListsStreamSet }

  pub fn devices(&self) -> DevicesSet { DevicesSet }

  pub fn e2e_cross_signing_keys(&self) -> E2eCrossSigningKeysSet { E2eCrossSigningKeysSet }

  pub fn e2e_cross_signing_signatures(&self) -> E2eCrossSigningSignaturesSet { E2eCrossSigningSignaturesSet }

  pub fn e2e_device_keys_json(&self) -> E2eDeviceKeysJsonSet { E2eDeviceKeysJsonSet }

  pub fn e2e_fallback_keys_json(&self) -> E2eFallbackKeysJsonSet { E2eFallbackKeysJsonSet }

  pub fn e2e_one_time_keys_json(&self) -> E2eOneTimeKeysJsonSet { E2eOneTimeKeysJsonSet }

  pub fn e2e_room_keys(&self) -> E2eRoomKeysSet { E2eRoomKeysSet }

  pub fn e2e_room_keys_versions(&self) -> E2eRoomKeysVersionsSet { E2eRoomKeysVersionsSet }

  pub fn erased_users(&self) -> ErasedUsersSet { ErasedUsersSet }

  pub fn event_auth(&self) -> EventAuthSet { EventAuthSet }

  pub fn event_auth_chain_links(&self) -> EventAuthChainLinksSet { EventAuthChainLinksSet }

  pub fn event_auth_chain_to_calculate(&self) -> EventAuthChainToCalculateSet { EventAuthChainToCalculateSet }

  pub fn event_auth_chains(&self) -> EventAuthChainsSet { EventAuthChainsSet }

  pub fn event_backward_extremities(&self) -> EventBackwardExtremitiesSet { EventBackwardExtremitiesSet }

  pub fn event_edges(&self) -> EventEdgesSet { EventEdgesSet }

  pub fn event_expiry(&self) -> EventExpirySet { EventExpirySet }

  pub fn event_forward_extremities(&self) -> EventForwardExtremitiesSet { EventForwardExtremitiesSet }

  pub fn event_json(&self) -> EventJsonSet { EventJsonSet }

  pub fn event_labels(&self) -> EventLabelsSet { EventLabelsSet }

  pub fn event_push_actions(&self) -> EventPushActionsSet { EventPushActionsSet }

  pub fn event_push_actions_staging(&self) -> EventPushActionsStagingSet { EventPushActionsStagingSet }

  pub fn event_push_summary(&self) -> EventPushSummarySet { EventPushSummarySet }

  pub fn event_push_summary_last_receipt_stream_id(&self) -> EventPushSummaryLastReceiptStreamIdSet { EventPushSummaryLastReceiptStreamIdSet }

  pub fn event_push_summary_stream_ordering(&self) -> EventPushSummaryStreamOrderingSet { EventPushSummaryStreamOrderingSet }

  pub fn event_relations(&self) -> EventRelationsSet { EventRelationsSet }

  pub fn event_reports(&self) -> EventReportsSet { EventReportsSet }

  pub fn event_search(&self) -> EventSearchSet { EventSearchSet }

  pub fn event_to_state_groups(&self) -> EventToStateGroupsSet { EventToStateGroupsSet }

  pub fn event_txn_id(&self) -> EventTxnIdSet { EventTxnIdSet }

  pub fn events(&self) -> EventsSet { EventsSet }

  pub fn ex_outlier_stream(&self) -> ExOutlierStreamSet { ExOutlierStreamSet }

  pub fn federation_inbound_events_staging(&self) -> FederationInboundEventsStagingSet { FederationInboundEventsStagingSet }

  pub fn federation_stream_position(&self) -> FederationStreamPositionSet { FederationStreamPositionSet }

  pub fn ignored_users(&self) -> IgnoredUsersSet { IgnoredUsersSet }

  pub fn insertion_event_edges(&self) -> InsertionEventEdgesSet { InsertionEventEdgesSet }

  pub fn insertion_event_extremities(&self) -> InsertionEventExtremitiesSet { InsertionEventExtremitiesSet }

  pub fn insertion_events(&self) -> InsertionEventsSet { InsertionEventsSet }

  pub fn instance_map(&self) -> InstanceMapSet { InstanceMapSet }

  pub fn local_current_membership(&self) -> LocalCurrentMembershipSet { LocalCurrentMembershipSet }

  pub fn local_media_repository(&self) -> LocalMediaRepositorySet { LocalMediaRepositorySet }

  pub fn local_media_repository_thumbnails(&self) -> LocalMediaRepositoryThumbnailsSet { LocalMediaRepositoryThumbnailsSet }

  pub fn local_media_repository_url_cache(&self) -> LocalMediaRepositoryUrlCacheSet { LocalMediaRepositoryUrlCacheSet }

  pub fn monthly_active_users(&self) -> MonthlyActiveUsersSet { MonthlyActiveUsersSet }

  pub fn open_id_tokens(&self) -> OpenIdTokensSet { OpenIdTokensSet }

  pub fn partial_state_events(&self) -> PartialStateEventsSet { PartialStateEventsSet }

  pub fn partial_state_rooms(&self) -> PartialStateRoomsSet { PartialStateRoomsSet }

  pub fn partial_state_rooms_servers(&self) -> PartialStateRoomsServersSet { PartialStateRoomsServersSet }

  pub fn presence(&self) -> PresenceSet { PresenceSet }

  pub fn presence_stream(&self) -> PresenceStreamSet { PresenceStreamSet }

  pub fn profiles(&self) -> ProfilesSet { ProfilesSet }

  pub fn push_rules(&self) -> PushRulesSet { PushRulesSet }

  pub fn push_rules_enable(&self) -> PushRulesEnableSet { PushRulesEnableSet }

  pub fn push_rules_stream(&self) -> PushRulesStreamSet { PushRulesStreamSet }

  pub fn pusher_throttle(&self) -> PusherThrottleSet { PusherThrottleSet }

  pub fn pushers(&self) -> PushersSet { PushersSet }

  pub fn ratelimit_override(&self) -> RatelimitOverrideSet { RatelimitOverrideSet }

  pub fn receipts_graph(&self) -> ReceiptsGraphSet { ReceiptsGraphSet }

  pub fn receipts_linearized(&self) -> ReceiptsLinearizedSet { ReceiptsLinearizedSet }

  pub fn received_transactions(&self) -> ReceivedTransactionsSet { ReceivedTransactionsSet }

  pub fn redactions(&self) -> RedactionsSet { RedactionsSet }

  pub fn refresh_tokens(&self) -> RefreshTokensSet { RefreshTokensSet }

  pub fn registration_tokens(&self) -> RegistrationTokensSet { RegistrationTokensSet }

  pub fn rejections(&self) -> RejectionsSet { RejectionsSet }

  pub fn remote_media_cache(&self) -> RemoteMediaCacheSet { RemoteMediaCacheSet }

  pub fn remote_media_cache_thumbnails(&self) -> RemoteMediaCacheThumbnailsSet { RemoteMediaCacheThumbnailsSet }

  pub fn room_account_data(&self) -> RoomAccountDataSet { RoomAccountDataSet }

  pub fn room_alias_servers(&self) -> RoomAliasServersSet { RoomAliasServersSet }

  pub fn room_aliases(&self) -> RoomAliasesSet { RoomAliasesSet }

  pub fn room_depth(&self) -> RoomDepthSet { RoomDepthSet }

  pub fn room_memberships(&self) -> RoomMembershipsSet { RoomMembershipsSet }

  pub fn room_retention(&self) -> RoomRetentionSet { RoomRetentionSet }

  pub fn room_stats_current(&self) -> RoomStatsCurrentSet { RoomStatsCurrentSet }

  pub fn room_stats_earliest_token(&self) -> RoomStatsEarliestTokenSet { RoomStatsEarliestTokenSet }

  pub fn room_stats_state(&self) -> RoomStatsStateSet { RoomStatsStateSet }

  pub fn room_tags(&self) -> RoomTagsSet { RoomTagsSet }

  pub fn room_tags_revisions(&self) -> RoomTagsRevisionsSet { RoomTagsRevisionsSet }

  pub fn rooms(&self) -> RoomsSet { RoomsSet }

  pub fn server_keys_json(&self) -> ServerKeysJsonSet { ServerKeysJsonSet }

  pub fn server_signature_keys(&self) -> ServerSignatureKeysSet { ServerSignatureKeysSet }

  pub fn sessions(&self) -> SessionsSet { SessionsSet }

  pub fn state_events(&self) -> StateEventsSet { StateEventsSet }

  pub fn stats_incremental_position(&self) -> StatsIncrementalPositionSet { StatsIncrementalPositionSet }

  pub fn stream_ordering_to_exterm(&self) -> StreamOrderingToExtermSet { StreamOrderingToExtermSet }

  pub fn stream_positions(&self) -> StreamPositionsSet { StreamPositionsSet }

  pub fn threepid_guest_access_tokens(&self) -> ThreepidGuestAccessTokensSet { ThreepidGuestAccessTokensSet }

  pub fn threepid_validation_session(&self) -> ThreepidValidationSessionSet { ThreepidValidationSessionSet }

  pub fn threepid_validation_token(&self) -> ThreepidValidationTokenSet { ThreepidValidationTokenSet }

  pub fn ui_auth_sessions(&self) -> UiAuthSessionsSet { UiAuthSessionsSet }

  pub fn ui_auth_sessions_credentials(&self) -> UiAuthSessionsCredentialsSet { UiAuthSessionsCredentialsSet }

  pub fn ui_auth_sessions_ips(&self) -> UiAuthSessionsIpsSet { UiAuthSessionsIpsSet }

  pub fn user_daily_visits(&self) -> UserDailyVisitsSet { UserDailyVisitsSet }

  pub fn user_directory(&self) -> UserDirectorySet { UserDirectorySet }

  pub fn user_directory_search(&self) -> UserDirectorySearchSet { UserDirectorySearchSet }

  pub fn user_directory_stream_pos(&self) -> UserDirectoryStreamPosSet { UserDirectoryStreamPosSet }

  pub fn user_external_ids(&self) -> UserExternalIdsSet { UserExternalIdsSet }

  pub fn user_filters(&self) -> UserFiltersSet { UserFiltersSet }

  pub fn user_ips(&self) -> UserIpsSet { UserIpsSet }

  pub fn user_signature_stream(&self) -> UserSignatureStreamSet { UserSignatureStreamSet }

  pub fn user_stats_current(&self) -> UserStatsCurrentSet { UserStatsCurrentSet }

  pub fn user_threepid_id_server(&self) -> UserThreepidIdServerSet { UserThreepidIdServerSet }

  pub fn user_threepids(&self) -> UserThreepidsSet { UserThreepidsSet }

  pub fn users(&self) -> UsersSet { UsersSet }

  pub fn users_in_public_rooms(&self) -> UsersInPublicRoomsSet { UsersInPublicRoomsSet }

  pub fn users_pending_deactivation(&self) -> UsersPendingDeactivationSet { UsersPendingDeactivationSet }

  pub fn users_to_send_full_presence_to(&self) -> UsersToSendFullPresenceToSet { UsersToSendFullPresenceToSet }

  pub fn users_who_share_private_rooms(&self) -> UsersWhoSharePrivateRoomsSet { UsersWhoSharePrivateRoomsSet }

  pub fn worker_locks(&self) -> WorkerLocksSet { WorkerLocksSet }

}