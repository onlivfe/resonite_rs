#[derive(
	Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
/// Short description of a session's user.
pub struct SessionUser {
	/// The username of the user
	pub username: String,
	#[serde(rename = "userID")]
	/// Almost always exists, but rarely inexplicably missing
	pub id: Option<crate::id::User>,
	#[serde(rename = "userSessionId")]
	/// ID of the user's session..?
	pub session_id: Option<crate::id::UserSession>,
	/// If the user is focused on this session
	pub is_present: bool,
	#[serde(default)]
	/// The output device type of the user
	///
	/// Default is used if missing from API response
	pub output_device: crate::model::OutputDevice,
}
