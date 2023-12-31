#![doc(
	html_logo_url = "https://github.com/onlivfe/resonite_rs/raw/main/logo.png"
)]
//! WIP: Typed models for [Resonite's API](https://wiki.resonite.com) with serde support.
//!
//! Currently very WIP since Resonite isn't public as of writing.
//! ```

#![cfg_attr(nightly, feature(doc_auto_cfg))]
#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![deny(clippy::cargo)]
#![warn(missing_docs)]
#![deny(rustdoc::invalid_html_tags)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
// My project my choice, tabs are literally made for indentation, spaces not.
#![allow(clippy::tabs_in_doc_comments)]
// Not much can be done about it :/
// #![allow(clippy::multiple_crate_versions)]

/// The base path of the API
const API_BASE_URI: &str = "https://api.resonite.com";

/// No authenticatino for the API client
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NoAuthentication {}

#[cfg(feature = "api_client")]
pub mod api_client;
