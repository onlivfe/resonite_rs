use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, serde::rfc3339};

#[cfg_attr(
	feature = "borsh",
	derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[serde_with::serde_as]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// A Resonite record's submission to a group
pub struct Submission {
	// TODO: Unique ID newtype
	/// If the submission should be featured or not
	pub featured: bool,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	/// The ID of the user that enabled featuring this submission
	pub featured_by_user_id: Option<crate::id::User>,
	#[cfg_attr(
		feature = "borsh",
		borsh(
			serialize_with = "crate::util::borsh::time::optional_ser",
			deserialize_with = "crate::util::borsh::time::optional_de"
		)
	)]
	#[serde(rename = "featuredTimestamp")]
	#[serde(default)]
	#[serde(with = "crate::util::opt_rfc3339")]
	#[serde(skip_serializing_if = "Option::is_none")]
	/// When featuring this submission was enabled
	pub featured_time: Option<OffsetDateTime>,
	/// The id of the submission
	pub id: String,
	/// The group that this submission is to
	pub owner_id: crate::id::Group,
	#[cfg_attr(
		feature = "borsh",
		borsh(
			serialize_with = "crate::util::borsh::time::ser",
			deserialize_with = "crate::util::borsh::time::de"
		)
	)]
	#[serde(with = "rfc3339")]
	/// When the submission was created
	pub submission_time: OffsetDateTime,
	/// The ID of the user that created the submission
	pub submitted_by_id: crate::id::User,
	/// The name of the submitter
	pub submitted_by_name: String,
	/// The id of the record that this submission is for
	pub target_record_id: crate::id::Record,
}
