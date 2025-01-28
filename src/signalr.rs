//! Inefficient but good enough `SignalR` wrappers for `WebSockets`

// Everything is re-exported from here, and just organized to files
// for easier navigation & better development experience.
#![allow(clippy::module_name_repetitions)]
// Dunno
#![allow(clippy::collection_is_never_read)]

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

// Modified version of code licensed under MIT from
// https://github.com/yurivoronin/ngx-signalr-websocket/blob/ab6db75462e1a25306c2ffb821008649fd45d6e5/projects/ngx-signalr-websocket/src/lib/protocol.ts
#[repr(u8)]
// #[cfg_attr(
// 	feature = "borsh",
// 	derive(borsh::BorshSerialize, borsh::BorshDeserialize)
// )]
// #[borsh(use_discriminant = true)]
#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	Hash,
	Serialize,
	Deserialize,
	strum::Display,
	strum::AsRefStr,
	strum::VariantNames,
)]
#[serde(untagged)]
/// The message type
pub enum Message {
	/// Cancel RPC call
	CancelInvocation {
		#[serde(rename = "type")]
		/// A hack to force serde to have this as `"type":5`
		num: VariantNumber<5>,
		#[serde(flatten)]
		/// The data for the invocation cancelling
		data: CancelInvocation,
	},
	/// Closes connection
	Close {
		#[serde(rename = "type")]
		/// A hack to force serde to have this as `"type":7`
		num: VariantNumber<7>,
	},
	/// Invocation completed
	Completion {
		#[serde(rename = "type")]
		/// A hack to force serde to have this as `"type":3`
		num: VariantNumber<3>,
		// #[cfg_attr(
		// 	feature = "borsh",
		// 	borsh(
		// 		serialize_with = "crate::util::borsh::json::ser",
		// 		deserialize_with = "crate::util::borsh::json::de"
		// 	)
		// )]
		#[serde(flatten)]
		/// The data for the invocation completion
		data: serde_json::Value,
	},
	/// RPC call
	Invocation {
		#[serde(rename = "type")]
		/// A hack to force serde to have this as `"type":1`
		num: VariantNumber<1>,
		#[serde(flatten)]
		/// The data for the invocation
		data: Invocation,
	},
	/// Keep the connection alive
	Ping {
		#[serde(rename = "type")]
		/// A hack to force serde to have this as `"type":6`
		num: VariantNumber<6>,
	},
	/// RPC call with streaming
	StreamInvocation {
		#[serde(rename = "type")]
		/// A hack to force serde to have this as `"type":4`
		num: VariantNumber<4>,
		// #[cfg_attr(
		// 	feature = "borsh",
		// 	borsh(
		// 		serialize_with = "crate::util::borsh::json::ser",
		// 		deserialize_with = "crate::util::borsh::json::de"
		// 	)
		// )]
		#[serde(flatten)]
		/// The data for the stream invocation
		data: serde_json::Value,
	},
	/// Data
	StreamItem {
		#[serde(rename = "type")]
		/// A hack to force serde to have this as `"type":2`
		num: VariantNumber<2>,
		// #[cfg_attr(
		// 	feature = "borsh",
		// 	borsh(
		// 		serialize_with = "crate::util::borsh::json::ser",
		// 		deserialize_with = "crate::util::borsh::json::de"
		// 	)
		// )]
		#[serde(flatten)]
		/// The data for the stream item
		data: serde_json::Value,
	},
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
/// A hack to force serde to accept numbers as enum tags
pub struct VariantNumber<const V: u8>;

impl<const V: u8> Serialize for VariantNumber<V> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_u8(V)
	}
}

impl<'de, const V: u8> Deserialize<'de> for VariantNumber<V> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let value = u8::deserialize(deserializer)?;
		if value == V {
			Ok(Self)
		} else {
			Err(serde::de::Error::custom("Parsing variant number failed"))
		}
	}
}

#[cfg(feature = "borsh")]
impl<const V: u8> borsh::BorshSerialize for VariantNumber<V> {
	fn serialize<W: std::io::Write>(
		&self, writer: &mut W,
	) -> std::io::Result<()> {
		let num: u8 = V;
		borsh::BorshSerialize::serialize(&num, writer)
	}
}

#[cfg(feature = "borsh")]
impl<const V: u8> borsh::BorshDeserialize for VariantNumber<V> {
	fn deserialize_reader<R: std::io::Read>(
		reader: &mut R,
	) -> std::io::Result<Self> {
		let num: u8 = borsh::BorshDeserialize::deserialize_reader(reader)?;
		if num == V {
			Ok(Self)
		} else {
			Err(borsh::io::Error::other(std::io::Error::other(
				"Wrong number: ".to_owned() + &num.to_string(),
			)))
		}
	}
}

#[cfg(test)]
#[test]
fn message_serde() {
	let src = "{\"type\":6}";
	let ping: Message = serde_json::from_str(src).unwrap();
	assert_eq!(ping, Message::Ping { num: VariantNumber });
	let str = serde_json::to_string(&ping).unwrap();
	assert_eq!(src, str);
}

#[cfg(all(test, feature = "borsh"))]
#[test]
fn message_borsh() {
	//let ping_msg = Message::Ping { num: VariantNumber };

	//let mut buf = vec![];
	//borsh::BorshSerialize::serialize(&ping_msg, &mut buf).unwrap();
	//let mut slice = buf.as_slice();
	//let roundtrip_msg: Message =
	//	borsh::BorshDeserialize::deserialize(&mut slice).unwrap();

	//assert_eq!(ping_msg, roundtrip_msg);
}

#[cfg_attr(
	feature = "borsh",
	derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Invocation cancellation
pub struct CancelInvocation {
	/// The ID of the invocation
	pub invocation_id: String,
}

// #[cfg_attr(
// 	feature = "borsh",
// 	derive(borsh::BorshSerialize, borsh::BorshDeserialize)
// )]
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// An invocation
pub struct Invocation {
	/// Data of the invocation
	#[serde(flatten)]
	pub data: InvocationData,
	/// The ID of the invocation
	///
	/// See <https://learn.microsoft.com/en-us/javascript/api/@microsoft/signalr/invocationmessage?view=signalr-js-latest#@microsoft-signalr-invocationmessage-invocationid>
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub invocation_id: Option<String>,
}

#[repr(u8)]
// #[cfg_attr(
// 	feature = "borsh",
// 	derive(borsh::BorshSerialize, borsh::BorshDeserialize)
// )]
// #[cfg_attr(feature = "borsh", borsh(use_discriminant = true))]
#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	Hash,
	Serialize,
	Deserialize,
	strum::Display,
	strum::AsRefStr,
	strum::VariantNames,
)]
#[serde(tag = "target", content = "arguments")]
/// Data of an invocation
pub enum InvocationData {
	/// Debug data
	Debug((String,)),
	/// Data about a session update
	ReceiveSessionUpdate((Box<crate::model::SessionInfo>,)),
	/// Session removal data
	// #[cfg_attr(
	// 	feature = "borsh",
	// 	borsh(
	// 		serialize_with = "crate::util::borsh::json::tuple_ser",
	// 		deserialize_with = "crate::util::borsh::json::tuple_de"
	// 	)
	// )]
	RemoveSession((crate::id::Session, OffsetDateTime)),
	/// Not yet supported or failed serde parsing of the invocation
	// #[cfg_attr(
	// 	feature = "borsh",
	// 	borsh(
	// 		serialize_with = "crate::util::borsh::json::ser",
	// 		deserialize_with = "crate::util::borsh::json::de"
	// 	)
	// )]
	#[serde(untagged)]
	Unknown(serde_json::Value),
}

#[cfg(test)]
#[test]
fn invocation_data_serde() {
	let src = "{\"target\":\"Debug\",\"arguments\":[\"Test 123\"]}";
	let debug: InvocationData = serde_json::from_str(src).unwrap();
	assert_eq!(debug, InvocationData::Debug(("Test 123".to_owned(),)));
	let str = serde_json::to_string(&debug).unwrap();
	assert_eq!(src, str);

	let session_info = crate::model::SessionInfo {
		name: String::default(),
		description: String::default(),
		world: None,
		tags: vec![],
		id: crate::id::Session::try_from("S-example").unwrap(),
		normalized_id: String::default(),
		host_id: None,
		host_user_session_id: None,
		host_machine_id: String::default(),
		host_username: String::default(),
		compatibility_hash: String::default(),
		system_compatibility_hash: String::default(),
		data_model_assemblies: vec![],
		universe_id: String::default(),
		app_version: String::default(),
		is_headless_host: Default::default(),
		urls: vec![],
		users: vec![],
		thumbnail_url: None,
		joined_users: Default::default(),
		active_users: Default::default(),
		total_joined_users: Default::default(),
		total_active_users: Default::default(),
		max_users: Default::default(),
		is_mobile_friendly: Default::default(),
		session_begin_time: OffsetDateTime::now_utc(),
		last_update_time: OffsetDateTime::now_utc(),
		access_level: crate::model::SessionAccessLevel::Anyone,
		has_ended: Default::default(),
		hide_from_listing: Default::default(),
		broadcast_key: Default::default(),
		is_valid: Default::default(),
		parent_session_ids: vec![],
		nested_session_ids: vec![],
	};

	let msg = InvocationData::ReceiveSessionUpdate((Box::new(session_info),));
	let as_json = serde_json::to_string(&msg).unwrap();
	let parsed = serde_json::from_str(&as_json).unwrap();
	assert_eq!(msg, parsed);

	let msg = Message::Invocation {
		num: VariantNumber,
		data: Invocation { invocation_id: None, data: msg },
	};
	let as_json = dbg!(serde_json::to_string(&msg).unwrap());
	let parsed = serde_json::from_str(&as_json).unwrap();
	assert_eq!(msg, parsed);
}
