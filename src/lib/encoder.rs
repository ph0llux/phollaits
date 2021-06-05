/*************************************************************************
* ph0llux:c08c5261c7afd62c018cd50059b5ecd3fcb16cec279b4a666579932bf9864fc7
*************************************************************************/

// - external
use base64::{encode};

pub trait ToBase64 {
	/// method converts the input to a base64 encoded [String].
	/// # Example
	/// ```
	///
	///	extern crate phollaits;
	/// use phollaits::{ToBase64};
	///	fn main() {
	///		let a = "Teststring".to_string();
	///		asserteq!(a.base64(), "VGVzdHN0cmluZw==".to_string());
	///	}
	fn base64(self) -> String;
}

impl ToBase64 for String {
	fn base64(self) -> String {
		encode(self.as_bytes())
	}
}

impl ToBase64 for &str {
	fn base64(self) -> String {
		encode(self.as_bytes())
	}
}

impl ToBase64 for Vec<u8> {
	fn base64(self) -> String {
		encode(self)
	}
}