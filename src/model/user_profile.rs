use serde::{Deserialize, Serialize};

use super::RecordId;

#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Partial profile of a Resonite user.
pub struct UserProfile {
	/// The URI seems to be in a Resonite's own link format
	pub icon_url: Option<crate::AssetUrl>,
	/// Custom text of the user
	pub tagline: Option<String>,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	/// Badges to display for the user
	///
	/// Defaults to empty list
	pub display_badges: Vec<RecordId>,
	/// Custom text of the user
	pub description: Option<String>,
}
