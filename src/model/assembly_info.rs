use serde::{Deserialize, Serialize};

#[cfg_attr(
	feature = "borsh",
	derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Details about an a Resonite assembly
pub struct AssemblyInfo {
	/// Hash for compatibility checking of the assembly
	pub compatibility_hash: String,
	/// The name of the assembly
	pub name: String,
}
