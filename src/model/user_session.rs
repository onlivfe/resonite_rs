use serde::{Deserialize, Serialize};
use time::{serde::rfc3339, OffsetDateTime};

#[derive(Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// An users (login/authorization) session.
///
/// Not to be confused with a Neos session that's "an instance of a world".
/// This is the response to logging in for example.
///
/// The response from the API at POST `userSessions`.
pub struct UserSession {
	/// The Neos user that this session is for
	pub user_id: crate::id::User,
	/// The secret token of this session
	pub token: String,
	#[serde(rename = "created")]
	#[serde(with = "rfc3339")]
	/// When the user session was created
	pub creation_time: OffsetDateTime,
	#[serde(rename = "expire")]
	#[serde(with = "rfc3339")]
	/// When the user session is set to expire
	pub expiration_time: OffsetDateTime,
	/// Returned when creating a new session
	pub secret_machine_id_hash: Option<String>,
	/// Returned when creating a new session
	pub secret_machine_id_salt: Option<String>,
	/// Returned when creating a new session
	pub uid_hash: Option<String>,
	/// Returned when creating a new session
	pub uid_salt: Option<String>,
	/// If the user session has the remember me checked (lives longer)
	pub remember_me: bool,
	/// If the user session has is bound to the specific machine ID
	pub is_machine_bound: bool,
	/// Presumably an URL which can be used to log out
	pub logout_url: String,
	/// If the logout URL can be used client side.
	///
	/// False meaning can't be used by client to log out...?
	pub logout_url_client_side: bool,
	/// How the user session was originally created
	pub original_login_type: UserSessionLoginType,
	/// How many times the session has been used
	pub session_login_counter: u64,
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
/// The login type for an user's (auth) session
pub enum UserSessionLoginType {
	#[serde(rename = "UNKNOWN")]
	/// The login type is not known
	Unknown,
	/// The login was created with a password
	Password,
	/// The login was created with saml2
	Saml2,
	/// The login was created from a migration
	Migration,
}

#[derive(Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Config file data that's returned when requesting an user session
pub struct ConfigFileData {
	/// Supposedly path to where the config file should be stored
	pub path: String,
	/// Supposedly path to where the config file should be stored
	pub content: String,
}

impl std::fmt::Debug for ConfigFileData {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ConfigFileData")
			.field("path", &self.path)
			.field("content", &"*****")
			.finish()
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Result from the API when requesting an user session
pub struct UserSessionResult {
	#[serde(rename = "entity")]
	/// The user session, called `'entity'` in the API
	pub user_session: UserSession,
	/// The config files for the user session
	pub config_files: Vec<ConfigFileData>,
}
