use serde::{Deserialize, Serialize};
use time::{serde::rfc3339, OffsetDateTime};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// A message between two accounts
pub struct Message {
	/// An UUID prefixed with `MSG-`
	pub id: String,
	/// The owner, so most likely the logged in user
	pub owner_id: crate::id::User,
	/// The sender of the message
	pub sender_id: crate::id::User,
	/// If the user is focused on this session
	pub recipient_id: crate::id::User,
	/// The user session ID of the sender of the message
	pub sender_user_session_id: Option<crate::id::UserSession>,
	/// The contents of the message
	#[serde(flatten)]
	pub content: MessageContents,
	#[serde(with = "rfc3339")]
	/// When the message was sent
	pub send_time: OffsetDateTime,
	#[serde(with = "rfc3339")]
	/// When the message was sent
	pub last_update_time: OffsetDateTime,
	#[serde(default)]
	#[serde(with = "crate::util::opt_rfc3339")]
	/// When the message was sent
	pub read_time: Option<OffsetDateTime>,
	#[serde(default)]
	/// If the message was from a migration
	///
	/// Defaults to false if missing
	pub is_migrated: bool,
}

impl Message {
	#[must_use]
	/// Gets the ID recipient's if the owners ID doesn't match it, otherwise the
	/// sender's id is returned
	pub fn non_owner_id(&self) -> &crate::id::User {
		if self.owner_id == self.recipient_id {
			&self.sender_id
		} else {
			&self.recipient_id
		}
	}

	#[cfg(feature = "rand_util")]
	#[must_use]
	/// Creates a new message with a random id and time set to now
	pub fn new(
		content: MessageContents, owner_and_sender: crate::id::User,
		sender_user_session_id: Option<crate::id::UserSession>,
		recipient: crate::id::User,
	) -> Self {
		let now = OffsetDateTime::now_utc();

		Self {
			owner_id: owner_and_sender.clone(),
			sender_id: owner_and_sender,
			recipient_id: recipient,
			content,
			id: Self::new_id(),
			sender_user_session_id,
			send_time: now,
			last_update_time: now,
			read_time: None,
			is_migrated: false,
		}
	}

	#[cfg(feature = "rand_util")]
	#[must_use]
	/// Generates a new pseudorandom ID for a message
	pub fn new_id() -> String {
		"MSG-".to_owned() + &crate::util::random_ascii_string(24)
	}
}

#[serde_with::serde_as]
#[allow(clippy::module_name_repetitions)]
#[derive(
	Debug, Clone, PartialEq, Serialize, Deserialize, strum::VariantNames,
)]
#[serde(tag = "messageType", content = "content")]
/// The contents of a message combined with the `MessageType`
pub enum MessageContents {
	/// A normal message
	Text(String),
	/// A generic object Record message
	Object(
		#[serde_as(as = "serde_with::json::JsonString")] Box<crate::model::Record>,
	),
	/// Voice recording
	Sound(
		#[serde_as(as = "serde_with::json::JsonString")] Box<crate::model::Record>,
	),
	/// Invite to a session
	SessionInvite(
		#[serde_as(as = "serde_with::json::JsonString")]
		Box<crate::model::SessionInfo>,
	),
}
