use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

use super::{SessionAccessLevel, UserSessionType};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Statistics related to users/sessions/etc that are online
pub struct OnlineStatistics {
	#[serde(with = "rfc3339")]
	/// When the statistics were captured
	pub capture_timestamp: OffsetDateTime,
	#[serde(rename = "visibleSessionsByAccessLevel")]
	/// How many of each session type there is
	pub visible_session_counts_by_access_level: HashMap<SessionAccessLevel, u32>,
	#[serde(rename = "hiddenSessionsByAccessLevel")]
	/// How many of each session type there is
	pub hidden_session_counts_by_access_level: HashMap<SessionAccessLevel, u32>,
	#[serde(rename = "activeVisibleSessionsByAccessLevel")]
	/// How many of each session type there is
	pub active_visible_session_counts_by_access_level:
		HashMap<SessionAccessLevel, u32>,
	#[serde(rename = "activeHiddenSessionsByAccessLevel")]
	/// How many of each session type there is
	pub active_hidden_session_counts_by_access_level:
		HashMap<SessionAccessLevel, u32>,
	#[serde(rename = "registeredUsers")]
	/// How many registered users are currently active
	pub registered_user_count: u32,
	#[serde(rename = "presentUsers")]
	/// How many users are currently present
	pub present_user_count: u32,
	#[serde(rename = "awayUsers")]
	/// How many users are currently away
	pub away_user_count: u32,
	/// How many instances of sessions exist
	pub instance_count: u32,
	#[serde(rename = "usersInVR")]
	/// How many users are in VR
	pub vr_user_count: u32,
	#[serde(rename = "usersInScreen")]
	/// How many users are in screen mode
	pub screen_user_count: u32,
	#[serde(rename = "usersOnDesktop")]
	/// How many users are in screen mode
	pub desktop_user_count: u32,
	#[serde(rename = "usersOnMobile")]
	/// How many users are on mobile
	pub mobile_user_count: u32,
	#[serde(rename = "usersInVisiblePublicSessions")]
	/// How many users are on in visible public sessions
	pub visible_public_session_user_count: u32,
	#[serde(rename = "usersInVisibleSemiAccessibleSessions")]
	/// How many users are on in registered users/etc sessions
	pub visible_semi_accessible_session_user_count: u32,
	#[serde(rename = "usersInHiddenSessions")]
	/// How many users are on in hidden sessions
	pub hidden_session_user_count: u32,
	#[serde(rename = "usersInPrivateSessions")]
	/// How many users are on in private sessions
	pub private_session_user_count: u32,
	#[serde(rename = "usersBySessionAccessLevel")]
	/// How many of each user session type there is
	pub user_count_by_session_access_level: HashMap<SessionAccessLevel, u32>,
	#[serde(rename = "usersByClientType")]
	/// How many of each user session type there is
	pub user_count_by_client_type: HashMap<UserSessionType, u32>,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Statistics related to the cloud
pub struct CloudStatistics {
	#[serde(with = "rfc3339")]
	/// When the statistics were captured
	pub capture_timestamp: OffsetDateTime,
	/// Statistic about the jobs count
	pub asset_metadata_jobs: u32,
	/// Statistic about the jobs count
	pub asset_variant_jobs: u32,
	/// Statistic about how many asset variants there are
	pub computed_asset_variants: u32,
	/// Statistic about the jobs count
	pub record_preprocess_jobs: u32,
	/// Statistic about the jobs count
	pub upload_jobs: u32,
	/// Statistic about the queue
	pub migration_tasks_in_queue: f32,
	/// Statistic about the speed of the migrations
	pub migration_records_per_minute: f32,
}
