/************************************************************************
* pk:f3e6cdbe65422da7d4878e43365373946ffd89fff7964d8901ff865cd50d0aa2
************************************************************************/

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
}

impl<T: ToString> OptionExtensions<T> for Option<T> {
	/// method to allow a conversion of Option<str> to Option<String> directly.
	/// # Example to expand tilda
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
	/// 	let b = s.shellexpand();
	/// 	assert_eq!(b, "/home/ph0llux/projects/phollaits".to_string());
	/// }
	/// ```
	fn shellexpand(self) -> String;
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
}
