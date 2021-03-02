/************************************************************************
* pk:c2cea782eca8f3e118728142a15f7013860d10cf39c2da0e887b3d58226ad263
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