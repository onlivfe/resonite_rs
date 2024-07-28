use serde::{Deserialize, Serialize};

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
	/// A standard ban
	Standard,
	/// A soft ban
	Soft,
	/// A hard ban
	Hard,
}
