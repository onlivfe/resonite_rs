use serde::{Deserialize, Serialize};

#[cfg_attr(
	feature = "borsh",
	derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Details about a Resonite group.
pub struct Group {
	#[serde(rename = "adminUserId")]
	/// The U-username form of ID
	pub admin_id: crate::id::User,
	/// The G-groupname form of ID
	pub id: crate::id::Group,
	#[serde(default)]
	/// If the group was migrated
	///
	/// Defaults to false
	pub is_migrated: bool,
	/// The name of the group
	pub name: String,
	/// How much large is the group's storage quota.
	///
	/// Probably only exists for members/admins.
	pub quota_bytes: Option<u64>,
	/// How much storage quota the group has used.
	///
	/// Probably only exists for members/admins.
	pub used_bytes: Option<u64>,
}
