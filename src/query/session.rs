use racal::Queryable;
use serde::{Deserialize, Serialize};

use super::NoAuthentication;

/// Gets details of publicly listed sessions
pub struct Sessions;

// TODO: VecSkipError
impl Queryable<NoAuthentication, Vec<crate::model::SessionInfo>> for Sessions {
	fn url(&self, _: &NoAuthentication) -> String {
		format!("{}/sessions", crate::API_BASE_URI)
	}
}

/// Gets details of publicly listed sessions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionInfo {
	/// The ID of the session to query information about
	pub session_id: crate::id::Session,
}

impl SessionInfo {
	/// Creates a new session info query
	pub fn new(session_id: impl Into<crate::id::Session>) -> Self {
		Self { session_id: session_id.into() }
	}
}

impl Queryable<NoAuthentication, crate::model::SessionInfo> for SessionInfo {
	fn url(&self, _: &NoAuthentication) -> String {
		format!("{}/sessions/{}", crate::API_BASE_URI, self.session_id.as_ref())
	}
}
