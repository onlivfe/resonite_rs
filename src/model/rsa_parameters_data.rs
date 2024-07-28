use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// A Resonite users public RSA key pair...for... session authentication?
pub struct RSAParametersData {
	/// The exponent component of the RSA public key
	pub exponent: String,
	/// The modulus component of the RSA public key
	pub modulus: String,
	/// Who knows...
	pub p: String,
	/// Who knows...
	pub q: String,
	#[serde(rename = "DP")]
	/// Who knows...
	pub dp: String,
	#[serde(rename = "DQ")]
	/// Who knows...
	pub dq: String,
	/// Who knows...
	pub inverse_q: String,
	/// Who knows...
	pub d: String,
}
