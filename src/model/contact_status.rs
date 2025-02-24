use serde::{Deserialize, Serialize};

#[repr(u8)]
#[cfg_attr(
	feature = "borsh",
	derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "borsh", borsh(use_discriminant = true))]
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
	/// Friends
	Accepted = 1,
	/// User has been blocked
	Blocked = 4,
	/// Ignored the friendship request
	Ignored = 3,
	/// Not friends
	None = 0,
	/// The user has requested friendship
	Requested = 2,
	/// Apparently possible value too..?
	SearchResult = 5,
}
