/************************************************************************
* pk:0be5980330abddd340e3d6384103add1aa9ab63f32c48a8b4cc5d71cbcf0595b
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