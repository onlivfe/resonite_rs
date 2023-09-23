//! An optional API client feature using `reqwest`
//!
//! Besides using this, you could instead easily implement your own client using
//! a different HTTP library with the [`racal::Queryable`](racal::Queryable)
//! trait. Though this does additionally support unwrapping the message/data of
//! the `Resonite` API responses.
//!
//! If you're implementing your own API client, you need to implement two
//! possible API states:
//!
//! 1. [`resonite::NoAuthentication`](crate::NoAuthentication)
//!
//! > Doesn't require authentication but still needs to be rate limited
//! > properly.

use std::num::NonZeroU32;

use governor::{
	clock::DefaultClock,
	middleware::NoOpMiddleware,
	state::{InMemoryState, NotKeyed},
	Quota,
	RateLimiter,
};
pub use racal::reqwest::{ApiClient, ApiError};
use reqwest::{header::HeaderMap, Client, RequestBuilder};

use crate::NoAuthentication;

type NormalRateLimiter =
	RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>;

#[must_use]
fn http_rate_limiter() -> NormalRateLimiter {
	RateLimiter::direct(
		Quota::per_minute(NonZeroU32::try_from(60).unwrap())
			.allow_burst(NonZeroU32::try_from(2).unwrap()),
	)
}

/// The main API client without authentication
pub struct UnauthenticatedResonite {
	user_agent: String,
	http: Client,
	rate_limiter: NormalRateLimiter,
}

#[async_trait::async_trait]
impl ApiClient<NoAuthentication> for UnauthenticatedResonite {
	fn state(&self) -> &NoAuthentication { &NoAuthentication {} }

	fn client(&self) -> &reqwest::Client { &self.http }

	async fn before_request(
		&self, req: RequestBuilder,
	) -> Result<RequestBuilder, racal::reqwest::ApiError> {
		self.rate_limiter.until_ready().await;
		Ok(req)
	}
}

impl UnauthenticatedResonite {
	/// Creates an unauthenticated API client
	fn http_client(user_agent: &str) -> Result<Client, ApiError> {
		Ok(Client::builder().user_agent(user_agent).build()?)
	}

	/// Creates a new Resonite API client
	///
	/// # Errors
	///
	/// If deserializing user agent into a header fails
	pub fn new(user_agent: String) -> Result<Self, ApiError> {
		Ok(Self {
			http: Self::http_client(&user_agent)?,
			rate_limiter: http_rate_limiter(),
			user_agent,
		})
	}
}
