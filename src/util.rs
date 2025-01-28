//! Utilities to help with communicating with Resonite's API

pub mod opt_rfc3339 {
	//! Time serde for date time's RFC3339 where errors are converted to None.
	#![allow(clippy::unnecessary_wraps)]
	use serde::{Deserializer, Serializer};
	use time::{OffsetDateTime, serde::rfc3339};

	/// Deserializes data into an offset date time, ignoring errors
	///
	/// # Errors
	///
	/// Doesn't really, but serde signature requires it.
	pub fn deserialize<'a, D: Deserializer<'a>>(
		deserializer: D,
	) -> Result<Option<OffsetDateTime>, D::Error> {
		rfc3339::option::deserialize(deserializer).map_or_else(|_| Ok(None), Ok)
	}

	/// Serializes data into a possible offset date time
	///
	/// # Errors
	///
	/// If the underlying time crate's serializer errors
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

	// Hex
	const DICT: &[char; 16] = &[
		'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e',
		'f',
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

#[cfg(all(test, feature = "rand_util"))]
#[test]
fn uid_gen() {
	let uid = random_ascii_string(32);
	assert_eq!(dbg!(uid).chars().count(), 64);
}

/// Helpers to make nanoserde work with more advanced types
#[cfg(feature = "nanoserde_bin")]
pub mod nanoserde {
	use std::str::FromStr;

	use nanoserde::{DeBin, SerBin};

	/// Converts a JSON value to a string
	#[derive(DeBin, SerBin)]
	pub struct JsonValue(String);

	impl From<&serde_json::Value> for JsonValue {
		fn from(val: &serde_json::Value) -> Self { Self(val.to_string()) }
	}

	impl From<&JsonValue> for serde_json::Value {
		/// WARNING: no error handling is, will just default to UNIX epoch
		fn from(val: &JsonValue) -> Self {
			Self::from_str(&val.0).unwrap_or(Self::Null)
		}
	}

	impl From<&Vec<serde_json::Value>> for JsonValue {
		fn from(val: &Vec<serde_json::Value>) -> Self {
			Self(serde_json::Value::Array(val.to_owned()).to_string())
		}
	}

	impl From<&JsonValue> for Vec<serde_json::Value> {
		/// WARNING: no error handling is, will just default to UNIX epoch
		fn from(val: &JsonValue) -> Self {
			match serde_json::Value::from_str(&val.0)
				.unwrap_or(serde_json::Value::Null)
			{
				serde_json::Value::Array(v) => v,
				_ => vec![],
			}
		}
	}

	/// Convert to timestamp, turning errors into the UNIX epoch
	#[derive(DeBin, SerBin)]
	pub struct UtcTimestamp(i64);

	impl From<&time::OffsetDateTime> for UtcTimestamp {
		fn from(val: &time::OffsetDateTime) -> Self { Self(val.unix_timestamp()) }
	}

	impl From<&UtcTimestamp> for time::OffsetDateTime {
		/// WARNING: no error handling is, will just default to UNIX epoch
		fn from(val: &UtcTimestamp) -> Self {
			Self::from_unix_timestamp(val.0).unwrap_or(Self::UNIX_EPOCH)
		}
	}

	/// Convert to an optional timestamp, turning errors None
	#[derive(DeBin, SerBin)]
	pub struct OptionalUtcTimestamp(Option<i64>);

	impl From<&Option<time::OffsetDateTime>> for OptionalUtcTimestamp {
		fn from(val: &Option<time::OffsetDateTime>) -> Self {
			val.map_or(Self(None), |t| Self(Some(t.unix_timestamp())))
		}
	}

	impl From<&OptionalUtcTimestamp> for Option<time::OffsetDateTime> {
		fn from(val: &OptionalUtcTimestamp) -> Self {
			if let Some(t) = val.0 {
				if let Ok(t) = time::OffsetDateTime::from_unix_timestamp(t) {
					return Some(t);
				}
			}

			None
		}
	}
}
