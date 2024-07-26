use racal::Queryable;
use serde::{Deserialize, Serialize};

use super::Authenticating;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Authentication using a password for an user session
pub struct UserSessionPasswordAuthentication {
	/// The password
	pub password: String,
	#[serde(skip_serializing_if = "Option::is_none")] 
	/// An optional recovery code
	pub recovery_code: Option<String>,
}



#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Authentication using a session token for an user session
pub struct UserSessionTokenAuthentication {
	/// The password
	pub session_token: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(tag = "$type")]
#[serde(rename_all = "camelCase")]
/// Authentication for an user session query with any kind of auth
pub enum UserSessionAuthentication {
	/// Authentication using a password
	Password(UserSessionPasswordAuthentication),
	/// Authentication using a pre-existing session token
	SessionToken(UserSessionTokenAuthentication),
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// A login request body's data.
pub struct UserSession {
	/// The username which to request the user session for
	pub username: String,
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
}

#[cfg(test)]
#[test]
fn user_session_password_auth() {
	let expected_deserialized = UserSessionAuthentication::Password(UserSessionPasswordAuthentication {
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
	let expected_deserialized: UserSessionAuthentication = UserSessionAuthentication::SessionToken(UserSessionTokenAuthentication {
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
	let user_session_query: UserSession = serde_json::from_str(expected_string).unwrap();
	let received_string =
		serde_json::to_string_pretty(&user_session_query).unwrap();

	assert_eq!(expected_string, received_string);
}
