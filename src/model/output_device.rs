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
	Serialize,
	strum::FromRepr,
	strum::Display,
	strum::EnumString,
	strum::AsRefStr,
	strum::VariantNames,
)]
/// The type of output device that the user is using.
///
/// The API is inconsistent, sometimes representing this as a string and
/// sometimes as a number.
pub enum OutputDevice {
	/// In game camera
	Camera = 3,
	/// Desktop
	Screen = 1,
	/// Output device not known
	Unknown = 0,
	#[strum(to_string = "VR")]
	#[serde(rename = "VR")]
	/// Virtual Reality
	Vr = 2,
}

impl Default for OutputDevice {
	fn default() -> Self { Self::Unknown }
}

// Allow the OutputDevice to be either represented as a string or number in
// JSON.
impl<'de> Deserialize<'de> for OutputDevice {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::de::Deserializer<'de>,
	{
		struct OutputDeviceVisitor;

		impl serde::de::Visitor<'_> for OutputDeviceVisitor {
			type Value = OutputDevice;

			fn expecting(
				&self, formatter: &mut std::fmt::Formatter,
			) -> std::fmt::Result {
				formatter.write_str("a string or number matching the OutputDevice enum")
			}

			fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				OutputDevice::from_repr(v).ok_or_else(|| {
					serde::de::Error::invalid_value(
						serde::de::Unexpected::Unsigned(v.into()),
						&"enum u8 repr",
					)
				})
			}

			fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				u8::try_from(v).map(|v| self.visit_u8(v)).map_err(|_| {
					serde::de::Error::invalid_value(
						serde::de::Unexpected::Unsigned(v),
						&"enum u8 repr",
					)
				})?
			}

			fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				u8::try_from(v).map(|v| self.visit_u8(v)).map_err(|_| {
					serde::de::Error::invalid_value(
						serde::de::Unexpected::Signed(v),
						&"enum u8 repr",
					)
				})?
			}

			fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				use std::str::FromStr;

				OutputDevice::from_str(v).map_err(|_| {
					serde::de::Error::invalid_value(
						serde::de::Unexpected::Str(v),
						&"enum str repr",
					)
				})
			}
		}

		deserializer.deserialize_any(OutputDeviceVisitor)
	}
}

#[cfg(test)]
mod tests {
	use super::OutputDevice;

	#[test]
	fn deserialize_vr_num() {
		let input = "2";
		let output: OutputDevice =
			serde_json::from_str(input).expect("deserializing from num to work");
		assert_eq!(output, OutputDevice::Vr);
	}

	#[test]
	fn deserialize_vr_str() {
		let input = "\"VR\"";
		let output: OutputDevice =
			serde_json::from_str(input).expect("deserializing from str to work");
		assert_eq!(output, OutputDevice::Vr);
	}

	#[test]
	fn serialize_vr() {
		let input = OutputDevice::Vr;
		let output = serde_json::to_string(&input).expect("serializing VR to work");
		assert_eq!(&output, "\"VR\"");
	}

	#[test]
	fn camera() {
		let val = serde_json::to_string(&OutputDevice::Camera)
			.expect("serializing to work");
		assert_eq!(&val, "\"Camera\"");
		let val: OutputDevice =
			serde_json::from_str(&val).expect("deserializing from str to work");
		assert_eq!(&val, &OutputDevice::Camera);
		let val: OutputDevice =
			serde_json::from_str("3").expect("deserializing from num to work");
		assert_eq!(&val, &OutputDevice::Camera);
	}

	#[test]
	fn unknown() {
		let val = serde_json::to_string(&OutputDevice::Unknown)
			.expect("serializing to work");
		assert_eq!(&val, "\"Unknown\"");
		let val: OutputDevice =
			serde_json::from_str(&val).expect("deserializing from str to work");
		assert_eq!(&val, &OutputDevice::Unknown);
		let val: OutputDevice =
			serde_json::from_str("0").expect("deserializing from num to work");
		assert_eq!(&val, &OutputDevice::Unknown);
	}
}
