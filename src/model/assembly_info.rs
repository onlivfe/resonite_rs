use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Details about an a Resonite assembly
pub struct AssemblyInfo {
	/// The name of the assembly
	pub name: String,
	/// Hash for compatibility checking of the assembly
	pub compatibility_hash: String,
}
