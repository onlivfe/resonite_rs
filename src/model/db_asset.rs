#[cfg(feature = "nanoserde_bin")]
use nanoserde::{DeBin, SerBin};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "nanoserde_bin", derive(DeBin, SerBin))]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Details about an asset on Resonite
pub struct DBAsset {
	/// How large the asset is
	pub bytes: u64,
	/// The hash of the asset
	pub hash: String,
}
