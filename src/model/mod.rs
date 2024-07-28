//! Models of Resonite's API.

// Everything is based on Resonite's models, so not much can be done
#![allow(clippy::struct_excessive_bools)]
// Everything is re-exported from here, and just organized to files
// for easier navigation & better development experience.
#![allow(clippy::module_name_repetitions)]

mod assembly_info;
mod db_asset;
mod group;
mod message;
mod online_status;
mod output_device;
mod public_ban_type;
mod record;
mod record_id;
mod rsa_parameters_data;
mod session_access_level;
mod session_user;
mod sessions_info;
mod stats;
mod submission;
mod user;
mod user_profile;
mod user_session;
mod user_session_metadata;
mod user_status;

pub use assembly_info::*;
pub use db_asset::*;
pub use group::*;
pub use message::*;
pub use online_status::*;
pub use output_device::*;
pub use public_ban_type::*;
pub use record::*;
pub use record_id::*;
pub use rsa_parameters_data::*;
pub use session_access_level::*;
pub use session_user::*;
pub use sessions_info::*;
pub use stats::*;
pub use submission::*;
pub use user::*;
pub use user_profile::*;
pub use user_session::*;
pub use user_session_metadata::*;
pub use user_status::*;
