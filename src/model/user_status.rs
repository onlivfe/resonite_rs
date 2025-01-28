use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::UserSessionType;

#[cfg_attr(
	feature = "borsh",
	derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// A Resonite user's or friend's status.
pub struct UserStatus {
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	/// Only seems to exist when the user is online
	pub app_version: Option<String>,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	/// Only seems to exist when the user is online
	pub compatibility_hash: Option<String>,
	/// The index of the current session the user is in
	pub current_session_index: u32,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	/// Who knows?
	pub hash_salt: Option<String>,
	/// If the user is using a mobile client.
	pub is_mobile: bool,
	/// If the user is present or not
	pub is_present: bool,
	#[cfg_attr(
		feature = "borsh",
		borsh(
			serialize_with = "crate::util::borsh::time::optional_ser",
			deserialize_with = "crate::util::borsh::time::optional_de"
		)
	)]
	#[serde(rename = "lastPresenceTimestamp")]
	#[serde(default)]
	#[serde(with = "crate::util::opt_rfc3339")]
	#[serde(skip_serializing_if = "Option::is_none")]
	/// When the user was last present
	pub last_presence_time: Option<OffsetDateTime>,
	#[cfg_attr(
		feature = "borsh",
		borsh(
			serialize_with = "crate::util::borsh::time::optional_ser",
			deserialize_with = "crate::util::borsh::time::optional_de"
		)
	)]
	#[serde(rename = "lastStatusChange")]
	#[serde(default)]
	#[serde(with = "crate::util::opt_rfc3339")]
	#[serde(skip_serializing_if = "Option::is_none")]
	/// When the user's status last changed
	pub last_status_change_time: Option<OffsetDateTime>,
	/// "Online" / "Offline" and so on
	pub online_status: crate::model::OnlineStatus,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	/// The output device type of the user
	///
	/// Default is used if missing from API response
	pub output_device: crate::model::OutputDevice,
	/// Only seems to exist when the user is online
	#[serde(rename = "publicRSAKey")]
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub public_rsa_key: Option<crate::model::RSAParametersData>,
	/// Only seems to exist when the user is online
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	pub sessions: Vec<crate::model::UserSessionMetadata>,
	/// The ID of the user this status is for
	pub user_id: crate::id::User,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	/// The session ID of the user
	pub user_session_id: Option<crate::id::UserSession>,
	/// Session type of the user
	#[serde(rename = "sessionType")]
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub user_session_type: Option<UserSessionType>,
}
