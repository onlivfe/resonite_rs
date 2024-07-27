use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// A generic Resonite record, used for storage related things.
pub struct RecordId {
	#[serde(rename = "recordId")]
	/// The ID of the record (`R-{uuid}` for example)
	pub id: crate::id::Record,
	/// The ID of the owner (`U-{uuid}` or `G-{uuid}` for example)
	pub owner_id: crate::id::Owner,
}

impl Display for RecordId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "neosrec:///{}/{}", self.owner_id.as_ref(), self.id.as_ref())
	}
}
