#[serde_with::serde_as]
#[derive(
	Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
/// Short description of a session's user.
pub struct SessionUser {
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(rename = "userID")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	/// Almost always exists, but rarely inexplicably missing
	pub id: Option<crate::id::User>,
	/// If the user is focused on this session
	pub is_present: bool,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	/// The output device type of the user
	///
	/// Default is used if missing from API response
	pub output_device: crate::model::OutputDevice,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(rename = "userSessionId")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	/// ID of the user's session..?
	pub session_id: Option<crate::id::UserSession>,
	/// The username of the user
	pub username: String,
}
