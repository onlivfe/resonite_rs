//! Models of Resonite's API.

// Everything is based on Resonite's models, so not much can be done
#![allow(clippy::struct_excessive_bools)]
// Everything is re-exported from here, and just organized to files
// for easier navigation & better development experience.
#![allow(clippy::module_name_repetitions)]

mod assembly_info;
mod db_asset;
mod output_device;
mod record;
mod record_id;
mod session_access_level;
mod session_user;
mod sessions_info;
mod stats;
mod submission;
mod user_session;

pub use assembly_info::*;
pub use db_asset::*;
pub use output_device::*;
pub use record::*;
pub use record_id::*;
pub use session_access_level::*;
pub use session_user::*;
pub use sessions_info::*;
pub use stats::*;
pub use submission::*;
pub use user_session::*;
