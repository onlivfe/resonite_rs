//! Models of Resonite's API.

// Everything is based on Resonite's models, so not much can be done
#![allow(clippy::struct_excessive_bools)]
// Everything is re-exported from here, and just organized to files
// for easier navigation & better development experience.
#![allow(clippy::module_name_repetitions)]

mod record;
mod db_asset;
mod submission;
mod user_session;

pub use record::*;
pub use db_asset::*;
pub use submission::*;
pub use user_session::*;
