//! Inefficient but good enough `SignalR` wrappers for `WebSockets`

// Everything is re-exported from here, and just organized to files
// for easier navigation & better development experience.
#![allow(clippy::module_name_repetitions)]

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

// Modified version of code licensed under MIT from
// https://github.com/yurivoronin/ngx-signalr-websocket/blob/ab6db75462e1a25306c2ffb821008649fd45d6e5/projects/ngx-signalr-websocket/src/lib/protocol.ts
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
#[serde(tag = "type")]
/// The message type
pub enum Message {
	#[serde(rename = "1")]
	/// RPC call
	Invocation(Invocation),
	#[serde(rename = "2")]
	/// Data
	StreamItem(serde_json::Value),
	#[serde(rename = "3")]
	/// Invocation completed
	Completion(serde_json::Value),
	#[serde(rename = "4")]
	/// RPC call with streaming
	StreamInvocation(serde_json::Value),
	#[serde(rename = "5")]
	/// Cancel RPC call
	CancelInvocation(CancelInvocation),
	#[serde(rename = "6")]
	/// Keep the connection alive
	Ping,
	#[serde(rename = "7")]
	/// Closes connection
	Close,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CancelInvocation {
	pub invocation_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invocation {
	pub invocation_id: String,
	#[serde(flatten)]
	pub data: InvocationData,
}

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
pub enum InvocationData {
	ReceiveSessionUpdate(Box<(crate::model::SessionInfo,)>),
	Debug((String,)),
	RemoveSession((crate::id::Session, OffsetDateTime)),
	#[serde(untagged)]
	Unknown(serde_json::Value),
}
