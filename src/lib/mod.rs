/*************************************************************************
* ph0llux:5bade9a3fa0817dfbba6da19abaaa598628eff26f6f8616015c5ad79ae9c50de
*************************************************************************/
// 
// - STD
use std::io;

// 
// - external
extern crate data_encoding;
extern crate md5;
extern crate sha1;
extern crate sha2;
extern crate base64;

// 
// - internal
pub use archive::*;
pub use hash::*;
pub use stdext::*;
pub use converter::*;
pub use encoder::*;
pub use errors::*;

// 
// - modules
mod archive;
mod hash;
mod stdext;
mod converter;
mod encoder;
mod errors;

pub type Result<T> = std::result::Result<T, PhollaitsError>;


pub trait ToIOResult<T> {
	/// method to convert the underlying type to a [std::io::Result].
	fn to_io_result(self) -> io::Result<T>;
}

impl<T, E: ToString> ToIOResult<T> for std::result::Result<T, E> {
	/// # Example
	///	```
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
	/// ```
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