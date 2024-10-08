//! Models for querying the Resonite API via the HTTP/REST API

// Everything is based on Resonite's models, so not much can be done
#![allow(clippy::struct_excessive_bools)]
// Everything is re-exported from here, and just organized to files
// for easier navigation & better development experience.
#![allow(clippy::module_name_repetitions)]

use racal::FromApiState;
use serde::{Deserialize, Serialize};

mod contact;
mod group;
mod message;
mod session;
mod stats;
mod testing;
mod user;
mod user_session;

pub use contact::*;
pub use group::*;
pub use message::*;
pub use session::*;
pub use stats::*;
pub use testing::*;
pub use user::*;
pub use user_session::*;

/// [`racal::Queryable`](racal::Queryable)'s `RequiredApiState`.
///
/// Even unauthenticated requests to Resonite's API should take rate limits
/// into account, thus not using `()` for the API state.
pub struct NoAuthentication {}

impl racal::FromApiState<Self> for NoAuthentication {
	fn from_state(state: &Self) -> &Self { state }
}

impl racal::FromApiState<Authentication> for NoAuthentication {
	fn from_state(_: &Authentication) -> &Self { &Self {} }
}

/// [`racal::Queryable`](racal::Queryable)'s `RequiredApiState`.
///
/// With authentication
#[derive(PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct Authentication {
	/// The secret authentication token
	pub token: String,
	/// The user that the authentication token is for
	pub user_id: crate::id::User,
}

impl Authentication {
	#[must_use]
	/// Turns the authentication into the header that it generates
	pub fn to_header(&self) -> (&'static str, String) {
		(
			"Authorization",
			("res ".to_owned() + self.user_id.as_ref() + ":" + &self.token),
		)
	}
}

impl From<crate::model::UserSession> for Authentication {
	fn from(value: crate::model::UserSession) -> Self {
		Self { token: value.token, user_id: value.user_id }
	}
}

impl std::fmt::Debug for Authentication {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Authentication")
			.field("token", &"*****")
			.field("user_id", &self.user_id)
			.finish()
	}
}

impl FromApiState<Self> for Authentication {
	fn from_state(state: &Self) -> &Self { state }
}

#[serde_with::serde_as]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
/// [`racal::Queryable`](racal::Queryable)'s `RequiredApiState`.
///
/// Contains the data needed to actually request an user session.
/// Mixes headers and actual body data together, not an actual Resonite model.
pub struct Authenticating {
	#[serde(rename = "UID")]
	/// Unique identifier header.
	///
	/// Should be a SHA256 hash of the hardware.
	/// Could be any SHA256, but API will treat this as a different device based
	/// on the value of this.
	pub unique_machine_identifier: String,
	#[serde(rename = "TOTP")]
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	/// TOTP header.
	///
	/// Usually should be composed of just a few numbers.
	/// Only needed in some cases, with first requirement being having
	/// second factor authentication even enabled for the account.
	pub second_factor: Option<String>,
}

impl FromApiState<Self> for Authenticating {
	fn from_state(state: &Self) -> &Self { state }
}
