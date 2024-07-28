use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::ContactStatus;

#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Details about a friend/contact.
pub struct Contact {
	/// The U-username form of ID of whose friend the details are for.
	pub owner_id: crate::id::User,
	/// The U-username form of ID
	pub id: crate::id::User,
	#[serde(rename = "contactUsername")]
	/// The actual username of the contact/friend
	pub username: String,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	/// Possible alternatives to username
	///
	/// Defaulted to empty array
	pub alternate_usernames: Vec<String>,
	/// The status of the friendship
	pub contact_status: ContactStatus,
	/// If the contact has been accepted
	pub is_accepted: bool,
	#[serde(default)]
	#[serde(with = "crate::util::opt_rfc3339")]
	#[serde(skip_serializing_if = "Option::is_none")]
	/// When the latest message with the friend was at.
	///
	/// Wrong/Invalid dates such as `0001-01-01T00:00:00` are expressed as
	/// None.
	pub latest_message_time: Option<OffsetDateTime>,
	#[serde(skip_serializing_if = "Option::is_none")]
	/// The profile of the user
	pub profile: Option<crate::model::UserProfile>,
	#[serde(default)]
	/// If the contact was from a migration
	///
	/// Defaults to false if missing
	pub is_migrated: bool,
	#[serde(default)]
	/// If the contact has done the migration
	///
	/// Defaults to false if missing
	pub is_counterpart_migrated: bool,
}
