/*************************************************************************
* ph0llux:ff33ab063141c2f6864e2031c29ae23326ee42f90ca77ab29f025b0751e9c76e
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

impl ToBase64 for Vec<u8> {
	fn base64(self) -> String {
		encode(self)
	}
}