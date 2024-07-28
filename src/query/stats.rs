use racal::{Queryable, RequestMethod};

use super::NoAuthentication;

/// Gets statistics related to users/sessions/etc that are online
pub struct OnlineStatistics;

impl Queryable<NoAuthentication, crate::model::OnlineStatistics>
	for OnlineStatistics
{
	fn url(&self, _: &NoAuthentication) -> String {
		format!("{}/stats/onlineStats", crate::HTTP_BASE_URI)
	}
}

/// Gets statistics related to the cloud
pub struct CloudStatistics;

impl Queryable<NoAuthentication, crate::model::CloudStatistics>
	for CloudStatistics
{
	fn url(&self, _: &NoAuthentication) -> String {
		format!("{}/stats/cloudStats", crate::HTTP_BASE_URI)
	}
}

/// Gets statistics related to the cloud
pub struct NotifyInstanceOnline(pub crate::id::Machine);

impl Queryable<NoAuthentication, ()> for NotifyInstanceOnline {
	fn url(&self, _: &NoAuthentication) -> String {
		format!("{}/stats/instanceOnline/{}", crate::HTTP_BASE_URI, self.0.as_ref())
	}

	fn method(&self, _state: &NoAuthentication) -> RequestMethod {
		RequestMethod::Post
	}
}
