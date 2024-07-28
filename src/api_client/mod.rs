//! An optional API client features using `reqwest` for the HTTP parts,
//! and `signalrs-client-custom-auth` for [`SignalR`](https://dotnet.microsoft.com/en-us/apps/aspnet/signalr).
//!
//! Besides using this, you could instead easily implement your own client using
//! a different HTTP library with the [`racal::Queryable`](racal::Queryable)
//! trait. Though this does additionally support unwrapping the message/data of
//! the `Resonite` API responses.
//!
//! If you're implementing your own API client, you need to implement three
//! possible API states:
//!
//! 1. [`resonite::query::NoAuthentication`](crate::query::NoAuthentication)
//!
//! > Doesn't require authentication but still needs to be rate limited
//! > properly.
//!
//! 2.[`resonite::query::Authenticating`](crate::query::Authenticating)
//!
//! > Almost a sub-state of not having authentication,
//! > but requires `UID` & `TOTP` headers to be send.
//! > Used for logging in.
//!
//! 3. [`resonite::query::Authentication`](crate::query::Authentication)
//!
//! > Requires the `Authorization` header in addition to the rate limiting.

#[cfg(feature = "http_client")]
mod http;
#[cfg(feature = "http_client")]
pub use http::*;

#[cfg(feature = "signalr_client")]
mod signalr;
#[cfg(feature = "signalr_client")]
pub use signalr::*;

/// An error that may happen with an API query
#[derive(Debug)]
pub enum ApiError {
	/// An error happened with serialization
	Serde(serde_json::Error),
	/// An error happened with the HTTPS request
	#[cfg(feature = "http_client")]
	Http(reqwest::Error),
	/// An error happened with the WS connection
	#[cfg(feature = "signalr_client")]
	WebSocket(ezsockets::Error),
	/// An error happened with sending `SignalR` data
	#[cfg(feature = "signalr_client")]
	Other(String),
}

impl From<serde_json::Error> for ApiError {
	fn from(err: serde_json::Error) -> Self { Self::Serde(err) }
}

#[cfg(feature = "http_client")]
impl From<reqwest::Error> for ApiError {
	fn from(err: reqwest::Error) -> Self { Self::Http(err) }
}

#[cfg(feature = "http_client")]
impl From<racal::reqwest::ApiError> for ApiError {
	fn from(err: racal::reqwest::ApiError) -> Self {
		match err {
			racal::reqwest::ApiError::Reqwest(e) => Self::Http(e),
			racal::reqwest::ApiError::Serde(e) => Self::Serde(e),
		}
	}
}

#[cfg(feature = "signalr_client")]
impl From<ezsockets::Error> for ApiError {
	fn from(err: ezsockets::Error) -> Self { Self::WebSocket(err) }
}
