/*************************************************************************
* ph0llux:0ca23b7195b4232ff3bc905cfef1078eb61fda51c2e9840bff9d2c606e684625
*************************************************************************/
// 
// - STD
use std::io;

// 
// - external
extern crate data_encoding;
extern crate md5;
extern crate ring;
extern crate base64;

// 
// - internal
pub use archive::*;
pub use hash::*;
pub use stdext::*;
pub use converter::*;
pub use encoder::*;

// 
// - modules
mod archive;
mod hash;
mod stdext;
mod converter;
mod encoder;

pub trait ToIOResult<T> {
	/// method to convert the underlying type to a [std::io::Result].
	fn to_io_result(self) -> io::Result<T>;
}

impl<T, E: ToString> ToIOResult<T> for Result<T, E> {
	/// # Example
	///	```rust
	/// extern crate phollaits;
	/// use phollaits::*;
	/// use std::io;
	///
	/// fn main() -> io::Result<()> {
	/// 	let a = Some(1);
	///		println!("{:?}", a.to_io_result()?);
	///		Ok(())
	/// }
	/// ```
	fn to_io_result(self) -> io::Result<T> {
		match self {
			Ok(x) => Ok(x),
			Err(err) => Err(io::Error::new(io::ErrorKind::Other, err.to_string())),
		}
	}
}

impl<T> ToIOResult<T> for Option<T> {
	/// # Example
	/// ```rust
	/// extern crate phollaits;
	/// use phollaits::*;
	/// use std::io;
	///
	/// fn main() -> io::Result<()> {
	/// 	let a = Some(1);
	///		println!("{:?}", a.to_io_result()?);
	///		Ok(())
	/// }
	/// ```
	fn to_io_result(self) -> io::Result<T> {
		match self {
			Some(x) => Ok(x),
			None => Err(io::Error::new(io::ErrorKind::Other, NONE)),
		}
	}
}

const NONE: &str = "None";
const FORMAT_TILDA: &str = "~";
const ENV_VAR_HOME: &str = "HOME";