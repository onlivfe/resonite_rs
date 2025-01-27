#[cfg(feature = "nanoserde_bin")]
use nanoserde::{DeBin, SerBin};
use time::{OffsetDateTime, serde::rfc3339};

use super::AssemblyInfo;

#[cfg_attr(feature = "nanoserde_bin", derive(DeBin, SerBin))]
#[serde_with::serde_as]
#[derive(
	Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
/// A Resonite session, often called an instance on other platforms.
pub struct SessionInfo {
	/// Who can access the session
	pub access_level: crate::model::SessionAccessLevel,
	/// The amount of users that are focused on the session
	pub active_users: u8,
	/// The version of Resonite that session is hosting
	pub app_version: String,
	/// Streaming related probably?
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	pub broadcast_key: bool,
	#[serde(default)]
	/// A hash to check if the session is compatible (version, plugins, etc)
	///
	/// Defaults to an empty string if missing
	pub compatibility_hash: String,
	#[serde(default)]
	/// Assemblies to check for compatibility
	///
	/// Defaults to empty if missing
	pub data_model_assemblies: Vec<AssemblyInfo>,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	/// The description of the session
	///
	/// Defaulted to empty string if the API returns none for the session.
	pub description: String,
	/// If the session has ended
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default = "has_ended_default")]
	pub has_ended: bool,
	/// If the session is hidden from session listing
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	pub hide_from_listing: bool,
	#[serde(rename = "hostUserId")]
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	/// The ID of the session's host (`U-{uuid}` for example)
	pub host_id: Option<crate::id::User>,
	/// The ID of the session's host's machine (`{uuid}`)
	pub host_machine_id: String,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	/// The ID of the host user's session
	pub host_user_session_id: Option<crate::id::UserSession>,
	/// The username of the session's host
	pub host_username: String,
	#[serde(rename = "sessionId")]
	/// The ID of the session (`S-{uuid}` for example)
	pub id: crate::id::Session,

	#[serde(rename = "headlessHost")]
	/// If the host is a headless (server) instance or not.
	pub is_headless_host: bool,
	#[serde(rename = "mobileFriendly")]
	/// If the session is suitable for mobile clients
	pub is_mobile_friendly: bool,
	/// If the session is valid
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	pub is_valid: bool,
	/// The amount of users that have joined the session
	pub joined_users: u8,
	#[cfg_attr(
		feature = "nanoserde_bin",
		nserde(proxy = "crate::util::nanoserde::UtcTimestamp")
	)]
	#[serde(rename = "lastUpdate")]
	#[serde(with = "rfc3339")]
	/// When the session was last updated
	pub last_update_time: OffsetDateTime,
	/// The max limit of users in the session
	pub max_users: u8,
	/// The name of the session
	pub name: String,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	/// Sessions that are the child of this session
	///
	/// Defaulted to empty vector if the API returns none for the session.
	pub nested_session_ids: Vec<crate::id::Session>,
	#[serde(rename = "normalizedSessionId")]
	/// Normalized (capitalization) version of the session's id (`s-{uuid}` for
	/// example)
	pub normalized_id: String,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	/// Sessions that this session is a child of
	///
	/// Defaulted to empty vector if the API returns none for the session.
	pub parent_session_ids: Vec<crate::id::Session>,
	#[cfg_attr(
		feature = "nanoserde_bin",
		nserde(proxy = "crate::util::nanoserde::UtcTimestamp")
	)]
	#[serde(with = "rfc3339")]
	/// When the session began
	pub session_begin_time: OffsetDateTime,
	#[serde(default)]
	/// A hash to check if the session is compatible system wise
	///
	/// Defaults to an empty string if missing
	pub system_compatibility_hash: String,
	#[serde(default)]
	/// The tags of the session
	pub tags: Vec<String>,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	/// A link to the thumbnail of the session.
	///
	/// Can be `https://` or `neosdb://` for example
	pub thumbnail_url: Option<crate::AssetUrl>,
	/// Total of `active_users`...?
	pub total_active_users: u8,
	/// Total of `joined_users`..?
	pub total_joined_users: u8,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	/// Which Resonite "universe" the session is in
	///
	/// Defaults to empty string if missing
	pub universe_id: String,
	#[serde(rename = "sessionURLs")]
	/// Links to the session, in custom protocols such as `lnl-nat:///` and
	/// `res-steam://`
	pub urls: Vec<String>,
	#[serde(rename = "sessionUsers")]
	/// A list of the session's users very basic details.
	pub users: Vec<crate::model::SessionUser>,
	#[serde(rename = "correspondingWorldId")]
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	/// The ID of the session's world
	pub world: Option<crate::model::RecordId>,
}

// If the field is missing, it probably has ended...
const fn has_ended_default() -> bool { true }

#[must_use]
/// Tries to strip XML tags out of a string.
///
/// Not using an actual XML parser though, just a simple `<` and `>` character
/// search.
fn bad_xml_strip(str: &str) -> String {
	let start_indexes = str.match_indices('<');
	let end_indexes = str.match_indices('>');

	let mut stripped_name = str.to_owned();
	start_indexes.rev().zip(end_indexes.rev()).for_each(
		|((start, _), (end, _))| {
			if start < end {
				stripped_name.replace_range(start..=end, "");
			}
		},
	);

	stripped_name
}

impl SessionInfo {
	#[must_use]
	/// Tries to remove the XML notation-like parts from a session's name.
	///
	/// Note that this is imperfect and not using an actual XML parser to remain
	/// lightweight.
	pub fn stripped_name(&self) -> String { bad_xml_strip(&self.name) }
}

#[allow(clippy::too_many_lines)]
#[cfg(test)]
#[test]
fn session_info() {
	use serde_json::json;

	let json = json!(        {
			"name": "<color=yellow>Soko's Library</color> - Folders, Items, Avatars",
			"description": "Folders, Items, Avatars, collected for new player equipping.",
			"correspondingWorldId": {
					"recordId": "R-62ab89f9-7b06-4487-8e36-35f2e46647f8",
					"ownerId": "U-Lpsuchtie"
			},
			"tags": [
					"fukuro",
					"world",
					"library",
					"プリメロ工房",
					"photo",
					"public",
					"folder",
					"folders",
					"items",
					"avatars",
					"soko"
			],
			"sessionId": "S-U-1QMVJqtmCsC:soko_library",
			"normalizedSessionId": "s-u-1qmvjqtmcsc:soko_library",
			"hostUserId": "U-1QMVJqtmCsC",
			"hostUserSessionId": "ffa04206-bf29-4c02-98cd-2664ed6aaccb",
			"hostMachineId": "7rpx1gn4dojdinqdhhwddqmbwqq1dw1jjdzjstpm3yisss5poq5o",
			"hostUsername": "toaster_headless",
			"compatibilityHash": "flPfsJqNoHFSFmtE9Nml8g==",
			"systemCompatibilityHash": "Wdkgr1roe8mxg4hbQ8EeNQ==",
			"dataModelAssemblies": [
					{
							"name": "Elements.Assets",
							"compatibilityHash": "Wk+BcQBAZQudiVcIa5eRyQ=="
					},
					{
							"name": "Elements.Core",
							"compatibilityHash": "3XaWAwwz7srqNA3kWPtREA=="
					},
					{
							"name": "FrooxEngine",
							"compatibilityHash": "k6Bzkm46ALxejJ/He3tPCQ=="
					},
					{
							"name": "FrooxEngine.Store",
							"compatibilityHash": "K22/sKfODKjKTdTi7M/GnQ=="
					},
					{
							"name": "ProtoFlux.Nodes.Core",
							"compatibilityHash": "XxMkLAc6ulemxNEOJo7IOQ=="
					},
					{
							"name": "ProtoFlux.Nodes.FrooxEngine",
							"compatibilityHash": "w5eT+VlkgtJngh/EZmrhfg=="
					},
					{
							"name": "ProtoFluxBindings",
							"compatibilityHash": "xVTKyKC5GEjeT7iUKFUn5g=="
					},
					{
							"name": "SkyFrost.Base",
							"compatibilityHash": "te30htqkPqTW/nEKuspZsA=="
					},
					{
							"name": "SkyFrost.Base.Models",
							"compatibilityHash": "pi+qiFhdImZ42rvy8ybJnQ=="
					}
			],
			"universeId": null,
			"appVersion": "2024.7.25.1284",
			"headlessHost": true,
			"sessionURLs": [
					"lnl-nat://be2ae106c5a14ea58ed05664ab2306bd/S-U-1QMVJqtmCsC:soko_library"
			],
			"parentSessionIds": [],
			"nestedSessionIds": [],
			"sessionUsers": [
					{
							"username": "toaster_headless",
							"userID": "U-1QMVJqtmCsC",
							"userSessionId": null,
							"isPresent": false,
							"outputDevice": null
					}
			],
			"thumbnailUrl": null,
			"joinedUsers": 0,
			"activeUsers": 0,
			"totalJoinedUsers": 0,
			"totalActiveUsers": 0,
			"maxUsers": 10,
			"mobileFriendly": false,
			"sessionBeginTime": "2024-07-28T22:00:44.403632Z",
			"lastUpdate": "2024-07-28T23:25:47.013536Z",
			"accessLevel": "Anyone",
			"hideFromListing": false,
			"broadcastKey": null,
			"hasEnded": false,
			"isValid": true
	});

	serde_json::from_value::<SessionInfo>(json).unwrap();
}
