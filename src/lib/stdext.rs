/*************************************************************************
* ph0llux:4fcc63e8c9eac6fae126761c91ea5dca3e9f6303efb0efc6939d58f2c124774c
*************************************************************************/
//! stdext module
// - STD
use std::env;
use std::num::ParseIntError;
 
// - internal
use crate as phollaits;
use crate::{PhollaitsError, Result};

/// Trait contains some extensions for [bool].
pub trait BoolExtensions {
	/// method to reverse the value of a bool.
	/// # Example
	/// ```
	/// extern crate phollaits;
	/// use phollaits::{BoolExtensions};
	///
	/// fn main() {
	/// 	let mut a = true;
	/// 	a.reverse();
	/// 	assert_eq!(a, false);
	/// }
	/// ```
	fn reverse(&mut self);
}

impl BoolExtensions for bool {
	fn reverse(&mut self) {
		match &self {
			true => *self = false,
			false => *self = true,
		}
	}
}

/// Trait contains some extensions for [Option].
pub trait OptionExtensions<T> {
	fn to_string_option(self) -> Option<String>;
	fn to_string(self) -> String;
}

impl<T: ToString> OptionExtensions<T> for Option<T> {
	/// method to allow a conversion of Option<str> to Option<String> directly.
	/// # Example
	/// ```
	///
	/// extern crate phollaits;
	/// use phollaits::*;
	/// fn main() {
	/// 		let a = Some("Yes yes yeeesss!");
	/// 		assert_eq!(a, Some("Yes yes yeeesss!"));
	/// 		assert_eq!(a.to_string_option(), Some("Yes yes yeeesss!".to_string()))
	/// }
	/// ```
	fn to_string_option(self) -> Option<String> {
		match self {
			Some(x) => Some(x.to_string()),
			None => None,
		}
	}

	/// method to allow a conversion of Option<T> to String directly (if T implements the [ToString]-trait).
	/// # Example
	/// ```
	///
	/// extern crate phollaits;
	/// use phollaits::*;
	/// fn main() {
	/// 		let a = Some("test");
	///			assert_eq!("test".to_string(), a.to_string())
	/// }
	/// ```
	fn to_string(self) -> String {
		match self {
			Some(x) => x.to_string(),
			None => phollaits::NONE.to_string()
		}
	}
}

/// Trait contains some extensions for [String].
pub trait StringExt {
	/// method to allow shell-like expansions in [String].
	/// # Example to expand tilda
	/// ```no_run
	/// extern crate phollaits;
	/// use phollaits::*;
	///
	/// fn main() {
	/// 	let a = "~/projects/phollaits".to_string();
	/// 	let b = a.shellexpand();
	/// 	assert_eq!(b, "/home/ph0llux/projects/phollaits".to_string());
	/// }
	/// ```
	fn shellexpand(self) -> String;

	/// method to trims newline at the end of the string.
	/// # Example to expand tilda
	/// ```
	/// extern crate phollaits;
	/// use phollaits::*;
	///
	/// fn main() {
	/// 	let a = "This \n is \n a \n string\n\n\r\n".to_string();
	/// 	let b = a.trim_newline_end();
	/// 	assert_eq!(b, String::from("This \n is \n a \n string"));
	/// }
	/// ```
	fn trim_newline_end(&self) -> String;

	/// converts a 'hexified' String to a Vec of Bytes.
	/// # Example
	/// ```
	///	use phollaits::{StringExt, PhollaitsError};
	///
	/// fn main() -> Result<(), PhollaitsError> {
	/// 	let m = "6f5902ac237024bdd0c176cb93063dc4".to_string();
	/// 	let m_as_bytes = m.hex_to_bytes()?;
	/// 	assert_eq!(m_as_bytes, [111, 89, 2, 172, 35, 112, 36, 189, 208, 193, 118, 203, 147, 6, 61, 196]);
	/// 	Ok(())
	/// }
	/// ```
	fn hex_to_bytes(self) -> Result<Vec<u8>>;
}

impl StringExt for String {
	fn shellexpand(self) -> String {
		if self.contains(phollaits::FORMAT_TILDA) {
			match env::var_os(phollaits::ENV_VAR_HOME) {
				Some(x) => match x.to_str() {
					Some(x) => return self.replace(phollaits::FORMAT_TILDA, x),
					None => return self,
				},
				None => return self,
			}
		}
		self
	}

	fn trim_newline_end(&self) -> String {
		let mut trimmed_string = self.clone();
		loop {
			if trimmed_string.ends_with('\n') {
				trimmed_string.pop();
			} else if trimmed_string.ends_with('\r') {
				trimmed_string.pop();
			} else {
				break;
			}
		}
	    String::from(trimmed_string)
	}

	fn hex_to_bytes(self) -> Result<Vec<u8>> {
		fn inner_hex_to_bytes(s: &str) -> std::result::Result<u8, ParseIntError> {
		    u8::from_str_radix(s, 16).map(|n| n as u8)
		}
		fn get_vec(chunks: Vec<String>) -> std::result::Result<Vec<u8>, ParseIntError> {
	    	chunks.into_iter().map(|x| inner_hex_to_bytes(&x)).collect()
	    }
	    let mut chunks = Vec::new();
	    let mut z = self.chars().peekable();
	    while z.peek().is_some() {
	    	let chunk: String = z.by_ref().take(2).collect();
	    	chunks.push(chunk)
	    }
	    match get_vec(chunks) {
	    	Ok(x) => Ok(x),
	    	Err(e) => Err(PhollaitsError::from(e))
	    }
	}
}

impl StringExt for &str {
	fn shellexpand(self) -> String {
		if self.contains(phollaits::FORMAT_TILDA) {
			match env::var_os(phollaits::ENV_VAR_HOME) {
				Some(x) => match x.to_str() {
					Some(x) => return self.replace(phollaits::FORMAT_TILDA, x),
					None => return self.to_string(),
				},
				None => return self.to_string(),
			}
		}
		self.to_string()
	}
	
	fn trim_newline_end(&self) -> String {
		let mut trimmed_string = self.to_string();
		loop {
			if trimmed_string.ends_with('\n') {
				trimmed_string.pop();
			} else if trimmed_string.ends_with('\r') {
				trimmed_string.pop();
			} else {
				break;
			}
		}
	    String::from(trimmed_string)
	}
	fn hex_to_bytes(self) -> Result<Vec<u8>> {
		fn inner_hex_to_bytes(s: &str) -> std::result::Result<u8, ParseIntError> {
		    u8::from_str_radix(s, 16).map(|n| n as u8)
		}
		fn get_vec(chunks: Vec<String>) -> std::result::Result<Vec<u8>, ParseIntError> {
	    	chunks.into_iter().map(|x| inner_hex_to_bytes(&x)).collect()
	    }
	    let mut chunks = Vec::new();
	    let mut z = self.chars().peekable();
	    while z.peek().is_some() {
	    	let chunk: String = z.by_ref().take(2).collect();
	    	chunks.push(chunk)
	    }
	    match get_vec(chunks) {
	    	Ok(x) => Ok(x),
	    	Err(e) => Err(PhollaitsError::from(e))
	    }
	}
}

/// Trait contains some extensions for [Vec].
pub trait VecExt {
	/// method to convert into Vec<String>.
	fn to_vec_string(self) -> Vec<String>;
}

impl VecExt for Vec<&str> {
	/// method to convert Vec<&str> into Vec<String>.
	/// # Example
	/// ```
	/// extern crate phollaits;
	/// use phollaits::*;
	/// 
	///	const a: [&'static str; 3] = [ "a", "b", "c" ];
	///
	/// fn main() {
	/// 	let b = a.to_vec();
	///		let c = vec!("a".to_string(), "b".to_string(), "c".to_string());
	/// 	assert_eq!(b.to_vec_string(), c);
	/// }
	/// ```
	fn to_vec_string(self) -> Vec<String> {
		self.into_iter().map(Into::into).collect()
	}
}