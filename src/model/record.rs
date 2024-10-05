use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, serde::rfc3339};

#[serde_with::serde_as]
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// A Resonite record, used for all kinds of storage objects
pub struct Record {
	/// The id of the record
	pub id: crate::id::Record,
	/// The owner of the record
	pub owner_id: crate::id::Owner,
	/// The URI that this record points to
	pub asset_uri: crate::AssetUrl,
	// TODO: Support legacy fields
	/// The version of the  record
	pub version: RecordVersion,
	/// The user readable name of the record
	pub name: String,
	#[serde(default)]
	#[serde_as(deserialize_as = "serde_with::DefaultOnError")]
	/// The user readable description of the record
	///
	/// Defaults to an empty string if null/none in the API.
	pub description: String,
	// TODO: Determine if common cases -> enumify
	/// The type of the record
	pub record_type: String,
	#[serde(default)]
	#[serde_as(deserialize_as = "serde_with::DefaultOnError")]
	/// The user readable name of the owner
	///
	/// Defaults to an empty string if null/none in the API.
	pub owner_name: String,
	#[serde(default)]
	#[serde_as(deserialize_as = "serde_with::DefaultOnError")]
	/// The tags of the record
	pub tags: HashSet<String>,
	#[serde_as(deserialize_as = "serde_with::DefaultOnError")]
	#[serde(default)]
	/// The path to this record
	///
	/// Defaulted to empty string if it doesn't exist.
	pub path: String,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	/// The URI that this record's thumbnail is at
	pub thumbnail_uri: Option<crate::AssetUrl>,
	#[serde(with = "rfc3339")]
	/// When the record was last modified at
	pub last_modification_time: OffsetDateTime,
	#[serde(default)]
	#[serde(with = "crate::util::opt_rfc3339")]
	#[serde(skip_serializing_if = "Option::is_none")]
	/// When the record was created at
	pub creation_time: Option<OffsetDateTime>,
	#[serde(default)]
	#[serde(with = "crate::util::opt_rfc3339")]
	#[serde(skip_serializing_if = "Option::is_none")]
	/// When the record was first published at
	pub first_publish_time: Option<OffsetDateTime>,
	/// If the record has been deleted or not
	pub is_deleted: bool,
	/// If the record is public or not
	pub is_public: bool,
	/// If the record is intended for patrons
	pub is_for_patrons: bool,
	/// If the record should be publicly findable
	pub is_listed: bool,
	/// If the record should be publicly findable
	pub visits: u32,
	/// The rating of the record
	pub rating: f32,
	/// Number for random ordering
	pub random_order: u32,
	#[serde(default)]
	#[serde_as(deserialize_as = "serde_with::DefaultOnError")]
	/// The record's submissions to groups
	pub submissions: Vec<crate::model::Submission>,
	#[serde(default)]
	#[serde_as(deserialize_as = "serde_with::DefaultOnError")]
	#[serde(alias = "neosDBmanifest")]
	/// Details about the asset
	pub asset_manifest: Vec<crate::model::DBAsset>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Versioning for a record
pub struct RecordVersion {
	/// The version of the asset, in the global scope
	pub global_version: u32,
	/// The version of the asset, in the local scope
	pub local_version: u32,
	/// Who last modified the record
	pub last_modifying_user_id: crate::id::User,
	/// The machine ID of whoever last modified the record.
	///
	/// Might not always start with `M-` though.
	pub last_modifying_machine_id: String,
}
