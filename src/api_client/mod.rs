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
//! 1. [`resonite::query::NoAuthentication`](crate::query::NoAuthentication)
//!
//! > Doesn't require authentication but still needs to be rate limited
//! > properly.
//!
//! 2. [`resonite::model::UserSession`](crate::model::UserSession)
//!
//! > Requires the `Authorization` header in addition to the rate limiting.

use std::num::NonZeroU32;

use governor::{
	clock::DefaultClock,
	middleware::NoOpMiddleware,
	state::{InMemoryState, NotKeyed},
	Quota,
	RateLimiter,
};
pub use racal::reqwest::{ApiClient, ApiError};
use reqwest::{
	header::{HeaderMap, HeaderValue},
	Client,
	RequestBuilder,
};
use serde::{Deserialize, Serialize};

use crate::query::{Authenticating, Authentication, NoAuthentication};

type NormalRateLimiter =
	RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
/// Data needed to actually request an user session.
///
/// Mixes headers and actual body data together, not an actual Resonite model.
pub struct UserSessionQueryWithHeaders {
	/// The actual body of the request
	pub body: crate::query::UserSession,
	#[serde(flatten)]
	/// Headers & so on needed for authentication requests
	pub data: Authenticating,
}

#[must_use]
fn http_rate_limiter() -> NormalRateLimiter {
	// ~5 seconds per request sustained over one minute, allowing up to a request
	// per second in bursts.
	RateLimiter::direct(
		Quota::per_minute(NonZeroU32::try_from(12).unwrap())
			.allow_burst(NonZeroU32::try_from(5).unwrap()),
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

/// The main API client that's in the process of authentication
///
/// Created with a tuple of the unauthenticated client & authentication,
/// and can always be downgraded into an unauthenticated client.
pub struct AuthenticatingResonite {
	base: UnauthenticatedResonite,
	data: Authenticating,
}

impl From<(UnauthenticatedResonite, Authenticating)>
	for AuthenticatingResonite
{
	fn from(value: (UnauthenticatedResonite, Authenticating)) -> Self {
		Self { base: value.0, data: value.1 }
	}
}

impl From<AuthenticatingResonite> for UnauthenticatedResonite {
	fn from(value: AuthenticatingResonite) -> Self { value.base }
}

#[async_trait::async_trait]
impl ApiClient<Authenticating> for AuthenticatingResonite {
	fn state(&self) -> &Authenticating { &self.data }

	fn client(&self) -> &reqwest::Client { &self.base.http }

	async fn before_request(
		&self, mut req: RequestBuilder,
	) -> Result<RequestBuilder, racal::reqwest::ApiError> {
		self.base.rate_limiter.until_ready().await;
		req = req.header("UID", &self.data.unique_machine_identifier);
		if let Some(second_factor_token) = &self.data.second_factor {
			req = req.header("TOTP", second_factor_token);
		}
		

		Ok(dbg!(req))
	}
}

/// The main API client with authentication
pub struct AuthenticatedResonite {
	user_agent: String,
	http: Client,
	rate_limiter: NormalRateLimiter,
	auth: Authentication,
}

#[async_trait::async_trait]
impl ApiClient<Authentication> for AuthenticatedResonite {
	fn state(&self) -> &Authentication { &self.auth }

	fn client(&self) -> &reqwest::Client { &self.http }

	async fn before_request(
		&self, req: RequestBuilder,
	) -> Result<RequestBuilder, racal::reqwest::ApiError> {
		self.rate_limiter.until_ready().await;
		Ok(req)
	}
}

impl AuthenticatedResonite {
	/// Creates an API client
	fn http_client(
		user_agent: &str, auth: &Authentication,
	) -> Result<Client, ApiError> {
		use serde::ser::Error;

		let builder = Client::builder();
		let mut headers = HeaderMap::new();

		headers.insert(
			"Authorization",
			("res ".to_owned() + auth.user_id.as_ref() + ":" + &auth.token)
				.parse()
				.map_err(|_| {
				serde_json::Error::custom("Couldn't turn auth into a header")
			})?,
		);

		Ok(builder.user_agent(user_agent).default_headers(headers).build()?)
	}

	/// Removes authentication to the API client
	///
	/// # Errors
	///
	/// If deserializing user agent fails.
	pub fn downgrade(self) -> Result<UnauthenticatedResonite, ApiError> {
		Ok(UnauthenticatedResonite {
			http: UnauthenticatedResonite::http_client(&self.user_agent)?,
			rate_limiter: self.rate_limiter,
			user_agent: self.user_agent,
		})
	}

	/// Creates a new authenticated Resonite API client
	///
	/// # Errors
	///
	/// If deserializing user agent into a header fails
	pub fn new(
		user_agent: String, auth: impl Into<Authentication> + Send,
	) -> Result<Self, ApiError> {
		let auth = auth.into();
		Ok(Self {
			http: Self::http_client(&user_agent, &auth)?,
			rate_limiter: http_rate_limiter(),
			user_agent,
			auth,
		})
	}
}

impl UnauthenticatedResonite {
	/// Creates an unauthenticated API client
	fn http_client(user_agent: &str) -> Result<Client, ApiError> {
		let mut default_headers = HeaderMap::new();
		default_headers.insert(
			reqwest::header::ACCEPT,
			HeaderValue::from_static("application/json"),
		);
		Ok(
			Client::builder()
				.user_agent(user_agent)
				.default_headers(default_headers)
				.build()?,
		)
	}

	/// Adds authentication to the API client
	///
	/// # Errors
	///
	/// If deserializing user agent or authentication fails.
	pub fn upgrade(
		self, auth: impl Into<Authentication> + Send,
	) -> Result<AuthenticatedResonite, ApiError> {
		let auth = auth.into();
		Ok(AuthenticatedResonite {
			http: AuthenticatedResonite::http_client(&self.user_agent, &auth)?,
			rate_limiter: self.rate_limiter,
			user_agent: self.user_agent,
			auth,
		})
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
