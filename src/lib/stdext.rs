/*************************************************************************
* ph0llux:b33a719af9521e5a29876b3f57c7131a8c9f9c216a96d125b3b8ccdc8a11a1b5
*************************************************************************/
/************************************************************************
* pk:ce86018792133a7ee2ecf65b3685a0e237998978565d75557abbbffbd2d1ca58
************************************************************************/
//! stdext module
// - STD
use std::env;
 
// - internal
use crate as phollaits;

/// Trait contains some extensions for [bool].
pub trait BoolExtensions {
	/// method to reverse the value of a bool.
	/// # Example
	/// ```rust
	/// fn main() {
	/// 	let a = true;
	/// 	a.reverse();
	/// 	asserteq!(a, false);
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
	/// ```rust
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
	/// ```rust
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
	/// ```rust
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
	/// ```rust
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
}

/// Trait contains some extensions for [Vec].
pub trait VecExt {
	/// method to convert into Vec<String>.
	fn to_vec_string(self) -> Vec<String>;
}

impl VecExt for Vec<&str> {
	/// method to convert Vec<&str> into Vec<String>.
	/// # Example
	/// ```rust
	/// extern crate phollaits;
	/// use phollaits::*;
	/// 
	///	const a: [&'static str; 6] = [ "a", "b", "c" ];
	///
	/// fn main() {
	/// 	let b = a.to_vec;
	///		let c = vec!("a".to_string(), "b".to_string(), "c".to_string());
	/// 	assert_eq!(b.to_vec_string, c);
	/// }
	/// ```
	fn to_vec_string(self) -> Vec<String> {
		self.into_iter().map(Into::into).collect()
	}
}