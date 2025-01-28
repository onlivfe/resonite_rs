use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::{OffsetDateTime, serde::rfc3339};

#[cfg_attr(
	feature = "borsh",
	derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[serde_as]
#[derive(Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// An users (login/authorization) session.
///
/// Not to be confused with a Resonite session that's "an instance of a world".
/// This is the response to logging in for example.
///
/// The response from the API at POST `userSessions`.
pub struct UserSession {
	#[cfg_attr(
		feature = "borsh",
		borsh(
			serialize_with = "crate::util::borsh::time::ser",
			deserialize_with = "crate::util::borsh::time::de"
		)
	)]
	#[serde(rename = "created")]
	#[serde(with = "rfc3339")]
	/// When the user session was created
	pub creation_time: OffsetDateTime,
	#[cfg_attr(
		feature = "borsh",
		borsh(
			serialize_with = "crate::util::borsh::time::ser",
			deserialize_with = "crate::util::borsh::time::de"
		)
	)]
	#[serde(rename = "expire")]
	#[serde(with = "rfc3339")]
	/// When the user session is set to expire
	pub expiration_time: OffsetDateTime,
	/// If the user session has is bound to the specific machine ID
	pub is_machine_bound: bool,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	/// Presumably an URL which can be used to log out
	///
	/// Defaulted to an empty string if missing
	pub logout_url: String,
	/// If the logout URL can be used client side.
	///
	/// False meaning can't be used by client to log out...?
	pub logout_url_client_side: bool,
	/// How the user session was originally created
	pub original_login_type: UserSessionLoginType,
	/// If the user session has the remember me checked (lives longer)
	pub remember_me: bool,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	/// Returned when creating a new session
	pub secret_machine_id_hash: Option<String>,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	/// Returned when creating a new session
	pub secret_machine_id_salt: Option<String>,
	/// How many times the session has been used
	pub session_login_counter: u64,
	/// The secret token of this session
	pub token: String,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	/// Returned when creating a new session
	pub uid_hash: Option<String>,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	/// Returned when creating a new session
	pub uid_salt: Option<String>,
	/// The Resonite user that this session is for
	pub user_id: crate::id::User,
}

impl std::fmt::Debug for UserSession {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("UserSession")
			.field("user_id", &self.user_id)
			.field("token", &"*****")
			.field("creation_time", &self.creation_time)
			.field("expiration_time", &self.expiration_time)
			.field("secret_machine_id_hash", &"*****")
			.field("secret_machine_id_salt", &"*****")
			.field("uid_hash", &"*****")
			.field("uid_salt", &"*****")
			.field("remember_me", &self.remember_me)
			.field("is_machine_bound", &self.is_machine_bound)
			.field("logout_url", &"*****")
			.field("logout_url_client_side", &"*****")
			.field("original_login_type", &self.original_login_type)
			.field("session_login_counter", &self.session_login_counter)
			.finish()
	}
}

impl UserSession {
	#[must_use]
	/// The `Authorization` header required to use this `NeosUserSession`.
	pub fn auth_header(&self) -> String {
		"res ".to_owned() + self.user_id.as_ref() + ":" + &self.token
	}
}

#[repr(u8)]
#[cfg_attr(
	feature = "borsh",
	derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "borsh", borsh(use_discriminant = true))]
#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	Hash,
	Serialize,
	Deserialize,
	PartialOrd,
	Ord,
	strum::FromRepr,
	strum::Display,
	strum::EnumString,
	strum::AsRefStr,
	strum::VariantNames,
)]
/// The login type for an user's (auth) session
pub enum UserSessionLoginType {
	/// The login was created from a migration
	Migration = 3,
	/// The login was created with a password
	Password = 1,
	/// The login was created with saml2
	Saml2 = 2,
	#[serde(rename = "UNKNOWN")]
	/// The login type is not known
	Unknown = 0,
}

impl Default for UserSessionLoginType {
	fn default() -> Self { Self::Unknown }
}

#[cfg_attr(
	feature = "borsh",
	derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[derive(Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Config file data that's returned when requesting an user session
pub struct ConfigFileData {
	/// Supposedly path to where the config file should be stored
	pub content: String,
	/// Supposedly path to where the config file should be stored
	pub path: String,
}

impl std::fmt::Debug for ConfigFileData {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ConfigFileData")
			.field("path", &self.path)
			.field("content", &"*****")
			.finish()
	}
}

#[cfg_attr(
	feature = "borsh",
	derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[serde_with::serde_as]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Result from the API when requesting an user session
pub struct UserSessionResult {
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	/// The config files for the user session
	pub config_files: Vec<ConfigFileData>,
	#[serde(rename = "entity")]
	/// The user session, called `'entity'` in the API
	pub user_session: UserSession,
}

#[repr(u8)]
#[cfg_attr(
	feature = "borsh",
	derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "borsh", borsh(use_discriminant = true))]
#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	Hash,
	Serialize,
	Deserialize,
	PartialOrd,
	Ord,
	strum::FromRepr,
	strum::Display,
	strum::EnumString,
	strum::AsRefStr,
	strum::VariantNames,
)]
///Type of the user session
pub enum UserSessionType {
	/// Automated agent
	Bot = 4,
	/// Special chat client
	ChatClient = 3,
	/// Standard game client
	GraphicalClient = 1,
	/// Host of sessions
	Headless = 2,
	/// Unknown user session type
	Unknown = 0,
}

impl Default for UserSessionType {
	fn default() -> Self { Self::Unknown }
}
