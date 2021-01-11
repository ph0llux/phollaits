/************************************************************************
* pk:577b8535a4bf374a042b49c57c55c32401c53f59fabcfb8ba889910f9db51088
***********************************************************************/

// - external
use base64::encode;

pub trait ToBase64 {
	/// method converts the input to a base64 encoded [String].
	/// # Example
	/// ```rustc
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