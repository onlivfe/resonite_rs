use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, serde::rfc3339};

#[cfg_attr(
	feature = "borsh",
	derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Details about a Resonite user.
pub struct User {
	#[cfg_attr(
		feature = "borsh",
		borsh(
			serialize_with = "crate::util::borsh::time::optional_ser",
			deserialize_with = "crate::util::borsh::time::optional_de"
		)
	)]
	#[serde(default)]
	#[serde(with = "crate::util::opt_rfc3339")]
	#[serde(skip_serializing_if = "Option::is_none")]
	/// When the account ban expires
	pub account_ban_expiration: Option<OffsetDateTime>,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	/// Possible alternatives to the normalized username.
	/// Only visible when logged in.
	///
	/// Defaulted to empty array
	pub alternate_emails: Vec<String>,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	/// Possible alternatives to the normalized username
	///
	/// Defaulted to empty array
	pub alternate_normalized_names: Vec<String>,
	#[cfg_attr(
		feature = "borsh",
		borsh(
			serialize_with = "crate::util::borsh::time::optional_ser",
			deserialize_with = "crate::util::borsh::time::optional_de"
		)
	)]
	#[serde(default)]
	#[serde(with = "crate::util::opt_rfc3339")]
	#[serde(skip_serializing_if = "Option::is_none")]
	/// When is it the birthday of the user
	/// Presumably only visible when logged in.
	///
	/// Defaulted to empty.
	pub date_of_birth: Option<OffsetDateTime>,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	/// The email address of the user.
	/// Only visible when logged in.
	pub email: Option<String>,
	#[cfg_attr(
		feature = "borsh",
		borsh(
			serialize_with = "crate::util::borsh::json::vec_ser",
			deserialize_with = "crate::util::borsh::json::vec_de"
		)
	)]
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	/// Patreon related.
	///
	/// Defaulted to an empty list.
	pub entitlements: Vec<serde_json::Value>,
	/// The U-username form of ID
	pub id: crate::id::User,
	/// If the account is verified
	pub is_verified: bool,
	#[cfg_attr(
		feature = "borsh",
		borsh(
			serialize_with = "crate::util::borsh::time::optional_ser",
			deserialize_with = "crate::util::borsh::time::optional_de"
		)
	)]
	#[serde(default)]
	#[serde(with = "crate::util::opt_rfc3339")]
	#[serde(skip_serializing_if = "Option::is_none")]
	/// When the listing ban expires
	pub listing_ban_expiration: Option<OffsetDateTime>,
	#[cfg_attr(
		feature = "borsh",
		borsh(
			serialize_with = "crate::util::borsh::json::ser",
			deserialize_with = "crate::util::borsh::json::de"
		)
	)]
	#[serde(default)]
	/// Migration related
	pub migrated_data: serde_json::Value,
	#[cfg_attr(
		feature = "borsh",
		borsh(
			serialize_with = "crate::util::borsh::time::optional_ser",
			deserialize_with = "crate::util::borsh::time::optional_de"
		)
	)]
	#[serde(default)]
	#[serde(with = "crate::util::opt_rfc3339")]
	#[serde(skip_serializing_if = "Option::is_none")]
	/// When the mute ban expires
	pub mute_ban_expiration: Option<OffsetDateTime>,
	/// Normalized (capitalization) version of the username.
	pub normalized_username: String,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	/// Old email addresses of the user.
	/// Presumably only visible when logged in.
	///
	/// Defaulted to empty array
	pub old_emails: Vec<String>,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	/// Old usernames of the user.
	/// Presumably only visible when logged in.
	///
	/// Defaulted to empty array
	pub old_usernames: Vec<String>,
	#[cfg_attr(
		feature = "borsh",
		borsh(
			serialize_with = "crate::util::borsh::json::ser",
			deserialize_with = "crate::util::borsh::json::de"
		)
	)]
	#[serde(default)]
	/// Data about the user's Patreon subscription
	pub patreon_data: serde_json::Value,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	/// The profile of the user
	pub profile: Option<crate::model::UserProfile>,
	#[cfg_attr(
		feature = "borsh",
		borsh(
			serialize_with = "crate::util::borsh::time::optional_ser",
			deserialize_with = "crate::util::borsh::time::optional_de"
		)
	)]
	#[serde(default)]
	#[serde(with = "crate::util::opt_rfc3339")]
	#[serde(skip_serializing_if = "Option::is_none")]
	/// When the public ban expires
	pub public_ban_expiration: Option<OffsetDateTime>,
	#[serde(skip_serializing_if = "Option::is_none")]
	/// The type of public ban
	pub public_ban_type: Option<crate::model::PublicBanType>,
	#[deprecated = "Marked as legacy"]
	#[serde_as(deserialize_as = "serde_with::DefaultOnError")]
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	/// How much large is the users storage quota.
	///
	/// The API returns -1 for no permissions, which is de-serialized into None
	/// here.
	pub quota_bytes: Option<u64>,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	/// If referred to Resonite by some other user
	pub referrer_user_id: Option<crate::id::User>,
	#[cfg_attr(
		feature = "borsh",
		borsh(
			serialize_with = "crate::util::borsh::time::ser",
			deserialize_with = "crate::util::borsh::time::de"
		)
	)]
	#[serde(rename = "registrationDate")]
	#[serde(with = "rfc3339")]
	/// When the user registered their Resonite account.
	pub registration_time: OffsetDateTime,
	#[cfg_attr(
		feature = "borsh",
		borsh(
			serialize_with = "crate::util::borsh::time::optional_ser",
			deserialize_with = "crate::util::borsh::time::optional_de"
		)
	)]
	#[serde(default)]
	#[serde(with = "crate::util::opt_rfc3339")]
	#[serde(skip_serializing_if = "Option::is_none")]
	/// When the spectator ban expires
	pub spectator_ban_expiration: Option<OffsetDateTime>,
	#[cfg_attr(
		feature = "borsh",
		borsh(
			serialize_with = "crate::util::borsh::json::vec_ser",
			deserialize_with = "crate::util::borsh::json::vec_de"
		)
	)]
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	/// Patreon related.
	///
	/// Defaulted to an empty list.
	pub supporter_metadata: Vec<serde_json::Value>,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	/// Tags of the user. Seem to match up with the badges.
	pub tags: Vec<String>,
	// TODO: ID type
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(rename = "uniqueDeviceIDs")]
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	/// When is it the birthday of the user
	/// Presumably only visible when logged in.
	///
	/// Defaulted to empty.
	pub unique_device_ids: Option<String>,
	#[deprecated = "Marked as legacy"]
	#[serde_as(deserialize_as = "serde_with::DefaultOnError")]
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	/// How much storage quota the user has used.
	///
	/// The API returns -1 for no permissions, which is de-serialized into None
	/// here.
	pub used_bytes: Option<u64>,
	/// The actual username
	pub username: String,
}
