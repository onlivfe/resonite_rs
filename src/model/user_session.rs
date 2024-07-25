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
	/// Presumably an URL which can be used to log out (only from client side?)
	pub logout_url_client_side: String,
	/// How the user session was originally created
	pub original_login_type: UserSessionLoginType,
	/// How many times the session has been used
	pub session_login_counter: u64
}

impl UserSession {
	#[must_use]
	/// The `Authorization` header required to use this `NeosUserSession`.
	pub fn auth_header(&self) -> String {
		"res ".to_owned() + self.user_id.as_ref() + ":" + &self.token
	}
}


#[derive(Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
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
	Migration
}
