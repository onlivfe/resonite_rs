use racal::Queryable;
use serde::{Deserialize, Serialize};

use super::NoAuthentication;

/// Gets details of publicly listed sessions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GroupInfo {
	/// The ID of the group to query information about
	pub group_id: crate::id::Group,
}

impl GroupInfo {
	/// Creates a new group info query
	pub fn new(group_id: impl Into<crate::id::Group>) -> Self {
		Self { group_id: group_id.into() }
	}
}

impl Queryable<NoAuthentication, crate::model::Group> for GroupInfo {
	fn url(&self, _: &NoAuthentication) -> String {
		format!("{}/groups/{}", crate::API_BASE_URI, self.group_id.as_ref())
	}
}
