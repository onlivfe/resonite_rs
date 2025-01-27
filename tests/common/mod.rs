#![cfg(feature = "http_client")]
// Something's funky with checking if these are used or not.
#![allow(dead_code)]

use std::sync::LazyLock;

use resonite::{
	api_client::{AuthenticatedResonite, UnauthenticatedResonite},
	model::UserSession,
	query::Authentication,
};

const USER_AGENT: &str = concat!(
	env!("CARGO_PKG_NAME"),
	"-TestRunner/",
	env!("CARGO_PKG_VERSION"),
	" (",
	env!("CARGO_PKG_REPOSITORY"),
	")",
);

pub fn api_no_auth() -> UnauthenticatedResonite {
	UnauthenticatedResonite::new(USER_AGENT.to_string()).unwrap()
}

pub static USER_SESSION: LazyLock<UserSession> = LazyLock::new(|| {
	#[cfg(feature = "nanoserde_bin")]
	{
		use nanoserde::DeBin;
		match std::fs::read("local/user-session.bin") {
			Ok(b) => match UserSession::deserialize_bin(&b) {
				Ok(user_session) => {
					assert!(!user_session.token.is_empty());
					assert!(!user_session.user_id.as_ref().is_empty());
					return user_session;
				}
				Err(e) => eprintln!(
					"Error parsing `local/user-session.bin`, falling back to JSON; {e}"
				),
			},
			Err(e) => {
				eprintln!("Missing `local/user-session.bin`, falling back to JSON; {e}")
			}
		}
	}

	let user_session: UserSession =
		serde_json::from_slice(&std::fs::read("local/user-session.json").expect(
			"must have a prepared `local/user-session.json` file for live API testing",
		))
		.expect("`local/user-session.json` file to parse into a user session");

	assert!(!user_session.token.is_empty());
	assert!(!user_session.user_id.as_ref().is_empty());

	user_session
});

pub fn api_auth() -> AuthenticatedResonite {
	let auth: Authentication = USER_SESSION.clone().into();
	AuthenticatedResonite::new(USER_AGENT.to_string(), auth).unwrap()
}

#[cfg(feature = "signalr_client")]
pub async fn api_signalr() -> resonite::api_client::ResoniteSignalRClient {
	let auth: Authentication = USER_SESSION.clone().into();
	resonite::api_client::ResoniteSignalRClient::new(USER_AGENT, &auth)
		.await
		.unwrap()
}
