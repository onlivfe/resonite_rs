#![doc(
	html_logo_url = "https://github.com/onlivfe/resonite_rs/raw/main/logo.png"
)]
//! Typed models for [Resonite's API](https://wiki.resonite.com) with serde support.
//!
//! Currently still missing a lot, as there's no official support or docs for
//! the API.

#![cfg_attr(nightly, feature(doc_auto_cfg))]
#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![deny(clippy::cargo)]
#![warn(missing_docs)]
#![deny(rustdoc::invalid_html_tags)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(rustdoc::missing_documentation)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::arbitrary_source_item_ordering)]
// My project my choice, tabs are literally made for indentation, spaces not.
#![allow(clippy::tabs_in_doc_comments)]
// Not much can be done about it :/
#![allow(clippy::multiple_crate_versions)]

/// The base path of the API
const HTTP_BASE_URI: &str = "https://api.resonite.com";
#[cfg(feature = "signalr_client")]
const SIGNALR_HUB_URI: &str = "wss://api.resonite.com/hub";

pub mod id;
pub mod model;
pub mod query;
pub mod util;

// The models are split into slightly smaller files in order to avoid a really
// long single file.
mod assets;
pub mod signalr;

// They are re-exported at the top level though to make importing them easier /
// less confusing.
pub use assets::*;

#[cfg(feature = "http_client")]
pub mod api_client;
