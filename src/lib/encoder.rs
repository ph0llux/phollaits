/*************************************************************************
* ph0llux:45edee0b2db9931f489c8b42741197b2c535ae968d87718e849d366545d345d8
*************************************************************************/

// - external
use base64::{encode};

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

impl ToBase64 for &str {
	fn base64(self) -> String {
		encode(self.as_bytes())
	}
}