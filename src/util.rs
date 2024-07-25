pub mod opt_rfc3339 {
	//! Time serde for date time's RFC3339 where errors are converted to None.
	#![allow(clippy::unnecessary_wraps)]
	use serde::{Deserializer, Serializer};
	use time::{serde::rfc3339, OffsetDateTime};

	pub fn deserialize<'a, D: Deserializer<'a>>(
		deserializer: D,
	) -> Result<Option<OffsetDateTime>, D::Error> {
		rfc3339::option::deserialize(deserializer).map_or_else(|_| Ok(None), Ok)
	}

	pub fn serialize<S: Serializer>(
		option: &Option<OffsetDateTime>, serializer: S,
	) -> Result<S::Ok, S::Error> {
		rfc3339::option::serialize(option, serializer)
	}
}

#[cfg(feature = "rand_util")]
#[must_use]
/// Generates a new (not cryptographically safe) pseudorandom string
///
/// The output string's char count is `bytes_count` multiplied by two
pub fn random_ascii_string(bytes_count: u8) -> String {
	// By using nanorand we avoid pulling in really heavy deps.
	use nanorand::Rng;

	const DICT: &[char; 16] = &[
		'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
		'p',
	];

	let mut bits = [0u8].repeat(bytes_count as usize);

	nanorand::tls_rng().fill(&mut bits);

	let mut string = String::with_capacity(bytes_count as usize * 2);

	for byte in bits {
		let first_char_index = byte & 0xF;
		let second_char_index = byte >> 4;
		string.push(DICT[first_char_index as usize]);
		string.push(DICT[second_char_index as usize]);
	}

	string
}
