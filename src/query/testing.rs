use racal::Queryable;

use super::NoAuthentication;

/// Pings the API
pub struct Ping;

impl Queryable<NoAuthentication, ()> for Ping {
	fn url(&self, _: &NoAuthentication) -> String {
		format!("{}/testing/ping", crate::API_BASE_URI)
	}

	fn deserialize(&self, _data: &[u8]) -> serde_json::Result<()> { Ok(()) }
}

/// Makes a health status check request
///
/// How this differs from ping is questionable
pub struct HealthCheck;

impl Queryable<NoAuthentication, ()> for HealthCheck {
	fn url(&self, _: &NoAuthentication) -> String {
		format!("{}/testing/healthCheck", crate::API_BASE_URI)
	}

	fn deserialize(&self, _data: &[u8]) -> serde_json::Result<()> { Ok(()) }
}
