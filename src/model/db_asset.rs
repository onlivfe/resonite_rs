use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Details about an asset on Resonite
pub struct DBAsset {
	/// How large the asset is
	pub bytes: u64,
	/// The hash of the asset
	pub hash: String,
}
