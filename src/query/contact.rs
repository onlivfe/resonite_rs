use racal::Queryable;

use super::Authentication;

/// Get the contacts/friends for a specific user
pub struct Contacts;

impl Queryable<Authentication, Vec<crate::model::Contact>> for Contacts {
	fn url(&self, auth: &Authentication) -> String {
		format!("{}/users/{}/contacts", crate::HTTP_BASE_URI, auth.user_id.as_ref())
	}
}
