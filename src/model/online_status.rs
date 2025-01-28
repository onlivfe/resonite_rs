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
	Deserialize,
	Serialize,
	strum::Display,
	strum::EnumString,
	strum::AsRefStr,
	strum::VariantNames,
)]
/// The online status of a Resonite user.
pub enum OnlineStatus {
	/// The user is away
	Away = 2,
	/// The user is busy offline
	Busy = 3,
	/// The user is invisible
	Invisible = 1,
	/// The user is offline
	Offline = 0,
	/// The user is online
	Online = 4,
	/// The user is sociable
	Sociable = 5,
}

impl Default for OnlineStatus {
	fn default() -> Self { Self::Offline }
}

impl OnlineStatus {
	/// (R,G,B) colors that are estimated from the game's UI
	#[must_use]
	pub const fn color(&self) -> (u8, u8, u8) {
		match &self {
			Self::Sociable => (97, 209, 250),
			Self::Online => (0, 255, 0),
			Self::Away => (255, 200, 0),
			Self::Busy => (255, 0, 0),
			Self::Offline | Self::Invisible => (127, 127, 127),
		}
	}
}
