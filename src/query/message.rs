#[cfg(feature = "nanoserde_bin")]
use nanoserde::{DeBin, SerBin};
use racal::Queryable;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::Authentication;

#[cfg_attr(feature = "nanoserde_bin", derive(DeBin, SerBin))]
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Query the messages for a specific user
pub struct Messages {
	#[cfg_attr(
		feature = "nanoserde_bin",
		nserde(proxy = "crate::util::nanoserde::OptionalUtcTimestamp")
	)]
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	/// If to only query messages that were sent after a certain time
	pub from_time: Option<OffsetDateTime>,
	/// Max amount of messages to retrieve,
	/// seems to be server side capped to 100 as of writing.
	pub max_amount: u16,
	/// If to fetch only unread messages
	pub unread_only: bool,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	/// If to only query messages with a certain user
	pub with_user: Option<crate::id::User>,
}

impl Default for Messages {
	/// Creates a messages query based on the ID
	fn default() -> Self {
		Self {
			max_amount: 100,
			unread_only: false,
			from_time: None,
			with_user: None,
		}
	}
}

impl Queryable<Authentication, Vec<crate::model::Message>> for Messages {
	fn url(&self, auth: &Authentication) -> String {
		let mut query = format!(
			"{}/users/{}/messages?maxItems={}",
			crate::HTTP_BASE_URI,
			auth.user_id.as_ref(),
			self.max_amount
		);

		if self.unread_only {
			query += "&unread=true";
		}

		if let Some(from_time) = self.from_time {
			query = query + "&fromTime=" + &from_time.to_string();
		}
		if let Some(with_user) = &self.with_user {
			query = query + "&user=" + with_user.as_ref();
		}

		query
	}
}

/// Send a message
impl Queryable<Authentication, Self> for crate::model::Message {
	fn url(&self, _: &Authentication) -> String {
		format!(
			"{}/users/{}/messages",
			crate::HTTP_BASE_URI,
			self.recipient_id.as_ref(),
		)
	}

	fn body(
		&self, _state: &Authentication,
	) -> Option<serde_json::Result<Vec<u8>>> {
		Some(serde_json::to_vec(self))
	}

	fn method(&self, _state: &Authentication) -> racal::RequestMethod {
		racal::RequestMethod::Post
	}
}
