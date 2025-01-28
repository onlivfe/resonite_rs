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

/// Helpers to make borsh work with more advanced types
#[cfg(feature = "borsh")]
pub mod borsh {
	/// `serde_json::Value` (de)serialization from borsh examples
	pub mod json {
		pub use de::deserialize_array as vec_de;
		pub use de::deserialize_value as de;
		pub use ser::serialize_array as vec_ser;
		pub use ser::serialize_value as ser;

		mod ser {
			use core::convert::TryFrom;

			use borsh::{
				BorshSerialize,
				io::{ErrorKind, Result, Write},
			};

			/// Serializes a JSON value
			///
			/// # Errors
			///
			/// If serialization fails
			pub fn serialize_value<W: Write>(
				value: &serde_json::Value, writer: &mut W,
			) -> Result<()> {
				match value {
					serde_json::Value::Null => 0_u8.serialize(writer),
					serde_json::Value::Bool(b) => {
						1_u8.serialize(writer)?;
						b.serialize(writer)
					}
					serde_json::Value::Number(n) => {
						2_u8.serialize(writer)?;
						serialize_number(n, writer)
					}
					serde_json::Value::String(s) => {
						3_u8.serialize(writer)?;
						s.serialize(writer)
					}
					serde_json::Value::Array(a) => {
						4_u8.serialize(writer)?;
						serialize_array(a, writer)
					}
					serde_json::Value::Object(o) => {
						5_u8.serialize(writer)?;
						serialize_map(o, writer)
					}
				}
			}

			fn serialize_number<W: Write>(
				number: &serde_json::Number, writer: &mut W,
			) -> Result<()> {
				// A JSON number can either be a non-negative integer (represented in
				// serde_json by a u64), a negative integer (by an i64), or a
				// non-integer (by an f64).
				// We identify these cases with the following single-byte discriminants:
				// 0 - u64
				// 1 - i64
				// 2 - f64
				if let Some(u) = number.as_u64() {
					0_u8.serialize(writer)?;
					return u.serialize(writer);
				}

				if let Some(i) = number.as_i64() {
					1_u8.serialize(writer)?;
					return i.serialize(writer);
				}

				if let Some(f) = number.as_f64() {
					2_u8.serialize(writer)?;
					return f.serialize(writer);
				}

				// technically, it should not be unreachable, but an error instead,
				// as assumption about unreachable depends on private implementation
				// detail but it's fine to leave it be unreachable! for an example
				unreachable!("number is neither a u64, i64, nor f64");
			}

			/// Serializes a vector of JSON values
			///
			/// # Errors
			///
			/// If serialization fails
			pub fn serialize_array<W: Write>(
				array: &Vec<serde_json::Value>, writer: &mut W,
			) -> Result<()> {
				// The implementation here is very similar to that of Vec<V>.
				writer.write_all(
					&(u32::try_from(array.len()).map_err(|_| ErrorKind::InvalidData)?)
						.to_le_bytes(),
				)?;
				for item in array {
					serialize_value(item, writer)?;
				}
				Ok(())
			}

			/// this is mutually recursive with `serialize_value`
			fn serialize_map<W: Write>(
				map: &serde_json::Map<String, serde_json::Value>, writer: &mut W,
			) -> Result<()> {
				// The implementation here is very similar to that of BTreeMap<K, V>.
				u32::try_from(map.len())
					.map_err(|_| ErrorKind::InvalidData)?
					.serialize(writer)?;

				for (key, value) in map {
					key.serialize(writer)?;
					serialize_value(value, writer)?;
				}

				Ok(())
			}
		}
		mod de {
			use borsh::{
				BorshDeserialize,
				io::{Error, ErrorKind, Read, Result},
			};

			/// this is copy-paste of <https://github.com/near/borsh-rs/blob/master/borsh/src/de/hint.rs#L2-L5>
			fn hint_cautious<T>(hint: u32) -> usize {
				#[allow(clippy::cast_possible_truncation)]
				let el_size = core::mem::size_of::<T>() as u32;
				core::cmp::max(core::cmp::min(hint, 4096 / el_size), 1) as usize
			}

			/// Deserializes a JSON value
			///
			/// # Errors
			///
			/// If deserialization fails
			pub fn deserialize_value<R: Read>(
				reader: &mut R,
			) -> Result<serde_json::Value> {
				let flag: u8 = BorshDeserialize::deserialize_reader(reader)?;
				match flag {
					0 => Ok(serde_json::Value::Null),
					1 => {
						let b: bool = BorshDeserialize::deserialize_reader(reader)?;
						Ok(serde_json::Value::Bool(b))
					}
					2 => {
						let n: serde_json::Number = deserialize_number(reader)?;
						Ok(serde_json::Value::Number(n))
					}
					3 => {
						let s: String = BorshDeserialize::deserialize_reader(reader)?;
						Ok(serde_json::Value::String(s))
					}
					4 => {
						let a: Vec<serde_json::Value> = deserialize_array(reader)?;
						Ok(serde_json::Value::Array(a))
					}
					5 => {
						let o: serde_json::Map<_, _> = deserialize_map(reader)?;
						Ok(serde_json::Value::Object(o))
					}
					_ => Err(Error::new(
						ErrorKind::InvalidData,
						format!(
							"Invalid JSON value representation: {flag}. The first byte must be 0-5",
						),
					)),
				}
			}

			fn deserialize_number<R: Read>(
				reader: &mut R,
			) -> Result<serde_json::Number> {
				let flag: u8 = BorshDeserialize::deserialize_reader(reader)?;
				match flag {
					0 => {
						let u: u64 = BorshDeserialize::deserialize_reader(reader)?;
						Ok(u.into())
					}
					1 => {
						let i: i64 = BorshDeserialize::deserialize_reader(reader)?;
						Ok(i.into())
					}
					2 => {
						let f: f64 = BorshDeserialize::deserialize_reader(reader)?;
						// This returns None if the number is a NaN or +/-Infinity,
						// which are not valid JSON numbers.
						serde_json::Number::from_f64(f).ok_or_else(|| {
							Error::new(
								ErrorKind::InvalidData,
								format!("Invalid JSON number: {f}"),
							)
						})
					}
					_ => Err(Error::new(
						ErrorKind::InvalidData,
						format!(
							"Invalid JSON number representation: {flag}. The first byte must be 0-2",
						),
					)),
				}
			}

			/// Deserializes a vector of JSON values
			///
			/// # Errors
			///
			/// If deserialization fails
			pub fn deserialize_array<R: Read>(
				reader: &mut R,
			) -> Result<Vec<serde_json::Value>> {
				// The implementation here is very similar to that of Vec<V>.
				let len = u32::deserialize_reader(reader)?;
				let mut result =
					Vec::with_capacity(hint_cautious::<(String, serde_json::Value)>(len));
				for _ in 0..len {
					let value = deserialize_value(reader)?;
					result.push(value);
				}
				Ok(result)
			}

			/// this is mutually recursive with `deserialize_value`
			fn deserialize_map<R: Read>(
				reader: &mut R,
			) -> Result<serde_json::Map<String, serde_json::Value>> {
				// The implementation here is very similar to that of BTreeMap<K, V>.

				let vec: Vec<(String, serde_json::Value)> = {
					// The implementation here is very similar to that of Vec<(K, V)>.
					let len = u32::deserialize_reader(reader)?;
					let mut result = Vec::with_capacity(hint_cautious::<(
						String,
						serde_json::Value,
					)>(len));
					for _ in 0..len {
						let pair = {
							let key = String::deserialize_reader(reader)?;
							let value = deserialize_value(reader)?;
							(key, value)
						};
						result.push(pair);
					}
					result
				};

				Ok(vec.into_iter().collect())
			}
		}
	}

	/// (de)serialization for the `time` crate.
	pub mod time {
		use borsh::BorshDeserialize;
		use borsh::BorshSerialize;
		use time::OffsetDateTime;

		/// Serializes a time to it's UNIX timestamp
		///
		/// # Errors
		///
		/// If serialization fails
		pub fn ser<W: borsh::io::Write>(
			obj: &OffsetDateTime, writer: &mut W,
		) -> Result<(), borsh::io::Error> {
			obj.unix_timestamp().serialize(writer)
		}

		/// Deserializes a time from it's UNIX timestamp
		///
		/// # Errors
		///
		/// If deserialization fails
		pub fn de<R: borsh::io::Read>(
			reader: &mut R,
		) -> Result<OffsetDateTime, borsh::io::Error> {
			let t: i64 = BorshDeserialize::deserialize_reader(reader)?;
			let res = OffsetDateTime::from_unix_timestamp(t);
			res.map_err(|e| borsh::io::Error::other(std::io::Error::other(e)))
		}

		/// Serializes an optional time to it's UNIX timestamp
		///
		/// # Errors
		///
		/// If serialization fails
		pub fn optional_ser<W: borsh::io::Write>(
			obj: &Option<OffsetDateTime>, writer: &mut W,
		) -> Result<(), borsh::io::Error> {
			match obj {
				None => 0_u8.serialize(writer),
				Some(t) => {
					1_u8.serialize(writer)?;
					t.unix_timestamp().serialize(writer)
				}
			}
		}

		/// Deserializes an optional time from it's possible UNIX timestamp
		///
		/// # Errors
		///
		/// If deserialization fails
		pub fn optional_de<R: borsh::io::Read>(
			reader: &mut R,
		) -> Result<Option<OffsetDateTime>, borsh::io::Error> {
			let flag: u8 = BorshDeserialize::deserialize_reader(reader)?;
			match flag {
				0 => Ok(None),
				1 => Ok(Some(de(reader)?)),
				_ => Err(borsh::io::Error::new(
					borsh::io::ErrorKind::InvalidData,
					format!(
						"Invalid optional datetime value representation: {flag}. The first byte must be 0-1",
					),
				)),
			}
		}
	}
}
