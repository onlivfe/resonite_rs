//! Models for querying the Resonite API

// Everything is based on Resonite's models, so not much can be done
#![allow(clippy::struct_excessive_bools)]
// Everything is re-exported from here, and just organized to files
// for easier navigation & better development experience.
#![allow(clippy::module_name_repetitions)]

use racal::FromApiState;
use serde::{Deserialize, Serialize};

//mod user_session;

//pub use user_session::*;

/// [`racal::Queryable`](racal::Queryable)'s `RequiredApiState`.
///
/// Even unauthenticated requests to Neos' API should take rate limits
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
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Authentication {
	#[serde(rename = "sessionToken")]
	/// The secret authentication token
	pub token: String,
	/// The user that the authentication token is for
	pub user_id: crate::id::User,
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
