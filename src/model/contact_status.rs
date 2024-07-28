use serde::{Deserialize, Serialize};
#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	Hash,
	Serialize,
	Deserialize,
	strum::Display,
	strum::EnumString,
	strum::AsRefStr,
	strum::VariantNames,
)]
/// The friendship status with a Resonite user
pub enum ContactStatus {
	/// Not friends
	None,
	/// Apparently possible value too..?
	SearchResult,
	/// The user has requested friendship
	Requested,
	/// Ignored the friendship request
	Ignored,
	/// User has been blocked
	Blocked,
	/// Accepted the user as a friend
	Accepted,
}
