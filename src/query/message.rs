use racal::Queryable;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::Authentication;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Query the messages for a specific user
pub struct Messages {
	/// Max amount of messages to retrieve,
	/// most likely capped server side to a smaller value
	pub max_amount: u16,
	/// If to fetch only unread messages
	pub unread_only: bool,
	/// If to only query messages that were sent after a certain time
	pub from_time: Option<OffsetDateTime>,
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

	fn deserialize(
		&self, data: &[u8],
	) -> serde_json::Result<Vec<crate::model::Message>> {
		let text = String::from_utf8_lossy(data);
		serde_json::from_str(&dbg!(text))
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

	fn deserialize(&self, data: &[u8]) -> serde_json::Result<Self> {
		let text = String::from_utf8_lossy(data);
		serde_json::from_str(&dbg!(text))
	}
}
