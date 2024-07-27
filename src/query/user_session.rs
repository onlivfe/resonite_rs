use racal::Queryable;
use serde::{Deserialize, Serialize};

use super::{Authenticating, Authentication};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Authentication using a password for an user session
pub struct UserSessionPasswordAuthentication {
	/// The password
	pub password: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	/// An optional recovery code
	pub recovery_code: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Authentication using a session token for an user session
pub struct UserSessionTokenAuthentication {
	/// The password
	pub session_token: String,
}

#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	Hash,
	Serialize,
	Deserialize,
	strum::AsRefStr,
	strum::VariantNames,
)]
#[serde(tag = "$type")]
#[serde(rename_all = "camelCase")]
/// Authentication for an user session query with any kind of auth
pub enum UserSessionAuthentication {
	/// Authentication using a password
	Password(UserSessionPasswordAuthentication),
	/// Authentication using a pre-existing session token
	SessionToken(UserSessionTokenAuthentication),
}

impl From<UserSessionTokenAuthentication> for UserSessionAuthentication {
	fn from(value: UserSessionTokenAuthentication) -> Self {
		Self::SessionToken(value)
	}
}

impl From<UserSessionPasswordAuthentication> for UserSessionAuthentication {
	fn from(value: UserSessionPasswordAuthentication) -> Self {
		Self::Password(value)
	}
}

#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	Hash,
	Serialize,
	Deserialize,
	strum::AsRefStr,
	strum::VariantNames,
)]
#[serde(rename_all = "camelCase")]
/// An identifier to use when requesting a session from the Resonite API.
///
/// Used when logging in for example in
/// [`LoginCredentials`](LoginCredentials::identifier).
pub enum LoginCredentialsIdentifier {
	/// Identify using the username
	Username(String),
	#[serde(rename = "ownerID")]
	/// Identify using the user's ID
	OwnerID(String),
	/// Identify using an email address
	Email(String),
}

impl LoginCredentialsIdentifier {
	#[must_use]
	/// Gets the inner string
	pub const fn inner(&self) -> &String {
		match self {
			Self::Username(s) | Self::Email(s) | Self::OwnerID(s) => s,
		}
	}

	#[must_use]
	/// Gets the inner string
	pub fn inner_mut(&mut self) -> &mut String {
		match self {
			Self::Username(s) | Self::Email(s) | Self::OwnerID(s) => s,
		}
	}

	#[must_use]
	/// If is username
	pub const fn is_username(&self) -> bool { matches!(self, Self::Username(_)) }

	#[must_use]
	/// If is email based
	pub const fn is_email(&self) -> bool { matches!(self, Self::Email(_)) }

	#[must_use]
	/// If is owner's ID based
	pub const fn is_ownerid(&self) -> bool { matches!(self, Self::OwnerID(_)) }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// A login request body's data.
pub struct UserSession {
	#[serde(flatten)]
	/// The way to identify the user account the request is for
	pub identifier: LoginCredentialsIdentifier,
	/// The authentication for the request
	pub authentication: UserSessionAuthentication,
	/// Can be a random UUID
	pub secret_machine_id: String,
	/// If the session should be valid for 30 days
	pub remember_me: bool,
}

impl Queryable<Authenticating, crate::model::UserSessionResult>
	for UserSession
{
	fn url(&self, _: &Authenticating) -> String {
		format!("{}/userSessions", crate::API_BASE_URI)
	}

	fn body(
		&self, _state: &Authenticating,
	) -> Option<serde_json::Result<Vec<u8>>> {
		Some(serde_json::to_vec(self))
	}

	fn method(&self, _: &Authenticating) -> racal::RequestMethod {
		racal::RequestMethod::Post
	}

	fn deserialize(
		&self, data: &[u8],
	) -> serde_json::Result<crate::model::UserSessionResult> {
		let value = String::from_utf8_lossy(data);
		serde_json::from_str(&dbg!(value))
	}
}

#[cfg(test)]
#[test]
fn user_session_password_auth() {
	let expected_deserialized =
		UserSessionAuthentication::Password(UserSessionPasswordAuthentication {
			password: "totally-my-password".to_string(),
			recovery_code: None,
		});

	let expected_str = r#"{
  "$type": "password",
  "password": "totally-my-password"
}"#;

	let auth_deserialized: UserSessionAuthentication =
		serde_json::from_str(expected_str).unwrap();
	assert_eq!(auth_deserialized, expected_deserialized);

	let serialized = serde_json::to_string_pretty(&auth_deserialized).unwrap();

	assert_eq!(expected_str, serialized);
}

#[cfg(test)]
#[test]
fn user_session_token_auth() {
	let expected_deserialized: UserSessionAuthentication =
		UserSessionAuthentication::SessionToken(UserSessionTokenAuthentication {
			session_token: "totally-legit-token".to_string(),
		});

	let expected_str = r#"{
  "$type": "sessionToken",
  "sessionToken": "totally-legit-token"
}"#;

	let auth_deserialized: UserSessionAuthentication =
		serde_json::from_str(expected_str).unwrap();
	assert_eq!(auth_deserialized, expected_deserialized);

	let serialized = serde_json::to_string_pretty(&auth_deserialized).unwrap();

	assert_eq!(expected_str, serialized);
}

#[cfg(test)]
#[test]
fn user_session() {
	let expected_string = r#"{
  "username": "ljoonal",
  "authentication": {
    "$type": "password",
    "password": "totally-my-password"
  },
  "secretMachineId": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
  "rememberMe": true
}"#;
	let user_session_query: UserSession =
		serde_json::from_str(expected_string).unwrap();
	let received_string =
		serde_json::to_string_pretty(&user_session_query).unwrap();

	assert_eq!(expected_string, received_string);
}

/// Tries to make the current authentication session last longer
pub struct ExtendUserSession;

impl Queryable<Authentication, ()> for ExtendUserSession {
	fn url(&self, _: &Authentication) -> String {
		format!("{}/userSessions", crate::API_BASE_URI)
	}

	fn method(&self, _: &Authentication) -> racal::RequestMethod {
		racal::RequestMethod::Patch
	}

	fn deserialize(&self, _data: &[u8]) -> serde_json::Result<()> { Ok(()) }
}
