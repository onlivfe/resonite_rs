#![cfg(feature = "http_client")]
// Something's funky with checking if these are used or not.
#![allow(dead_code)]

use once_cell::sync::Lazy;
use resonite::{
	api_client::{
		AuthenticatedResonite,
		UnauthenticatedResonite,
		UserSessionQueryWithHeaders,
	},
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

pub static USER_SESSION: Lazy<UserSession> = Lazy::new(|| {
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
