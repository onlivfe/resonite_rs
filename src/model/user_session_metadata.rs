#[serde_with::serde_as]
#[derive(
	Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
/// A Resonite session, often called an instance on other platforms.
pub struct UserSessionMetadata {
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	/// A hash to check if the session is compatible (version, plugins, etc)
	///
	/// Defaults to an empty string if missing
	pub session_hash: String,
	/// Who can access the session
	pub access_level: crate::model::SessionAccessLevel,
	#[serde(rename = "sessionHidden")]
	/// If the session is hidden
	pub is_hidden: bool,
	/// If the user that this relates to is the host of the session
	pub is_host: bool,
	/// Streaming related probably?
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	pub broadcast_key: bool,
}
