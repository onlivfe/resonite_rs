use std::num::NonZeroU32;

use governor::{
	Quota,
	RateLimiter,
	clock::DefaultClock,
	middleware::NoOpMiddleware,
	state::{InMemoryState, NotKeyed},
};
pub use racal::reqwest::ApiClient;
use reqwest::{
	Client,
	RequestBuilder,
	header::{HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};

use super::ApiError;
use crate::query::{Authenticating, Authentication, NoAuthentication};

type NormalRateLimiter =
	RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>;

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
	http: Client,
	rate_limiter: NormalRateLimiter,
	user_agent: String,
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

		//Ok(dbg!(req))
		Ok(req)
	}
}

/// The main API client with authentication
pub struct AuthenticatedResonite {
	auth: Authentication,
	http: Client,
	rate_limiter: NormalRateLimiter,
	user_agent: String,
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

		let (header_name, header_value) = auth.to_header();

		headers.insert(
			header_name,
			header_value.parse().map_err(|_| {
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
