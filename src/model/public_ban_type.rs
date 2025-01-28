use serde::{Deserialize, Serialize};

#[repr(u8)]
#[cfg_attr(
	feature = "borsh",
	derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "borsh", borsh(use_discriminant = false))]
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
/// The type of a ban.
pub enum PublicBanType {
	/// A hard ban
	Hard = 2,
	/// A soft ban
	Soft = 1,
	/// A standard ban
	Standard = 0,
}
