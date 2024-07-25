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

#![allow(clippy::multiple_crate_versions)]

/// The base path of the API
const API_BASE_URI: &str = "https://api.resonite.com";

pub mod id;
pub mod model;
pub mod query;

// The models are split into slightly smaller files in order to avoid a really
// long single file.
mod assets;

// Models that should match up with Resonite' API's ones.

mod util;

// They are re-exported at the top level though to make importing them easier /
// less confusing.
pub use assets::*;

#[cfg(feature = "http_client")]
pub mod api_client;