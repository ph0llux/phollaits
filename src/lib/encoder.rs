/*************************************************************************
* ph0llux:24efd740f491bae39a06ceb74c1f642f8d93b8e4aebdf03f95e32031b4f7ceea
*************************************************************************/

// - external
use base64::{encode};
use hex::ToHex as HexToHex;

pub trait ToBase64 {
	/// method converts the input to a base64 encoded [String].
	/// # Example
	/// ```
	///
	///	extern crate phollaits;
	/// use phollaits::{ToBase64};
	///	fn main() {
	///		let a = "Teststring".to_string();
	///		assert_eq!(a.base64(), "VGVzdHN0cmluZw==".to_string());
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

pub trait ToHex {
	fn hexify(self) -> String;
	fn hexify_upper(self) -> String;
}

impl<T: AsRef<[u8]>> ToHex for T {
	/// converts a value (implementing AsRef<[u8]>; this includes String, str, Vec<u8> and [u8]) to hex.
	/// # Example
	/// ```
	/// use phollaits::{ToHex};
	///
	/// fn main() {
	///		let value = "myValue!";
	///		assert_eq!(value.hexify(), "6d7956616c756521".to_string())
	/// }
	/// ```
	fn hexify(self) -> String {
		self.encode_hex::<String>()
	}

	/// converts a value (implementing AsRef<[u8]>; this includes String, str, Vec<u8> and [u8]) to hex (UPPER_CASE).
	/// # Example
	/// ```
	/// use phollaits::{ToHex};
	///
	/// fn main() {
	///		let value = "myValue!";
	///		assert_eq!(value.hexify_upper(), "6D7956616C756521".to_string())
	/// }
	/// ```
	fn hexify_upper(self) -> String {
		self.encode_hex_upper::<String>()
	}
}