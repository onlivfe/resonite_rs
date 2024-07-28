use racal::Queryable;
use serde::{Deserialize, Serialize};

use super::NoAuthentication;

/// An user's ID or their username
///
/// Used in [`UserInfo`](resonite::query::UserInfo).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UserIdOrUsername {
	/// An user's ID
	Id(crate::id::User),
	/// An user's username
	Username(String),
}

impl UserIdOrUsername {
	#[must_use]
	/// If it's an ID
	pub const fn is_id(&self) -> bool { matches!(self, Self::Id(_)) }

	#[must_use]
	/// If it's an username
	pub const fn is_username(&self) -> bool { matches!(self, Self::Username(_)) }
}

impl AsRef<str> for UserIdOrUsername {
	fn as_ref(&self) -> &str {
		match self {
			Self::Id(v) => v.as_ref(),
			Self::Username(v) => v,
		}
	}
}

/// For easier scripting, should use String otherwise.
impl From<&'static str> for UserIdOrUsername {
	fn from(v: &'static str) -> Self { Self::Username(v.to_owned()) }
}

impl From<String> for UserIdOrUsername {
	fn from(v: String) -> Self { Self::Username(v) }
}

impl From<crate::id::User> for UserIdOrUsername {
	fn from(v: crate::id::User) -> Self { Self::Id(v) }
}

/// Gets details of an user by either username or ID
///
/// # Example usage
///
/// ```no_run
/// # tokio_test::block_on(async {
/// # use resonite::{api_client::{UnauthenticatedResonite, ApiClient}, query::UserSearch};
/// # let USER_AGENT = String::new();
/// # let resonite_api_client = UnauthenticatedResonite::new(USER_AGENT).unwrap();
/// let resonite_user_search_query = UserSearch::new("Resonite");
/// let resonite_bot = resonite_api_client
/// 	.query(resonite_user_search_query)
/// 	.await
/// 	.expect("to be able to get the Resonite bot account from Resonite");
/// println!(
/// 	"The Resonite bot supposedly registered on {}",
/// 	&resonite_bot.first().unwrap().registration_time
/// );
/// # })
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserInfo {
	/// The ID or username to query information about
	pub user: UserIdOrUsername,
}

impl UserInfo {
	/// Creates a new user info query based on the username or ID
	pub fn new(user: impl Into<UserIdOrUsername>) -> Self {
		Self { user: user.into() }
	}
}

impl Queryable<NoAuthentication, crate::model::User> for UserInfo {
	fn url(&self, _: &NoAuthentication) -> String {
		format!(
			"{}/users/{}?byUsername={}",
			crate::API_BASE_URI,
			self.user.as_ref(),
			&(!self.user.is_id()).to_string()
		)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Searches users by name
pub struct UserSearch {
	/// The name to search for
	pub name: String,
}

impl UserSearch {
	/// Creates a new user search query
	pub fn new(name: impl Into<String>) -> Self { Self { name: name.into() } }
}

impl Queryable<NoAuthentication, Vec<crate::model::User>> for UserSearch {
	fn url(&self, _: &NoAuthentication) -> String {
		format!("{}/users?name={}", crate::API_BASE_URI, self.name)
	}
}
