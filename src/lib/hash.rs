/*************************************************************************
* ph0llux:3a8f5e8522529c5e9e3fcf0e2013b21a1aae8665015986961cd830d286d0142d
*************************************************************************/
//!hash Module.

// - STD
use std::io;
use std::io::Read;

// - internal
use super::{PhollaitsError, PhollaitsErrorKind, Result};

// - external
use data_encoding::HEXLOWER;
use md5::{Md5};
use sha1::{Sha1};
use sha2::{Sha256, Sha384, Sha512};
use tar::Entry;

/// This trait implements several hash-algorithms for several types.
pub trait HashExt {
	/// this method returns the md5-digest for implemented types as a [std::io::Result]
	/// of [String].
	/// # Example
	/// ```rust
	/// extern crate phollaits;
	/// use phollaits::HashExt;
	/// use std::fs;
	/// use std::io;
	/// fn main() -> io::Result<()> {
	/// 	let file = fs::File::open("/home/ph0llux/Pictures/image01.png")?;
	/// 	println!("{:?}", file.md5sum());
	/// 	Ok(())
	/// }
	/// ```
	fn md5sum(&mut self) -> Result<String>;

	/// this method returns the sha1-digest for implemented types as a [std::io::Result]
	/// of [String].
	/// # Example
	/// ```rust
	/// extern crate phollaits;
	/// use phollaits::HashExt;
	/// use std::fs;
	/// use std::io;
	/// fn main() -> io::Result<()> {
	/// 	let file = fs::File::open("/home/ph0llux/Pictures/image01.png")?;
	/// 	println!("{:?}", file.sha1sum());
	/// 	Ok(())
	/// }
	/// ```
	fn sha1sum(&mut self) -> Result<String>;

	/// this method returns the sha256-digest for implemented types as a [std::io::Result]
	/// of [String].
	/// # Example
	/// ```rust
	/// extern crate phollaits;
	/// use phollaits::HashExt;
	/// use std::fs;
	/// use std::io;
	/// fn main() -> io::Result<()> {
	/// 	let file = fs::File::open("/home/ph0llux/Pictures/image01.png")?;
	/// 	println!("{:?}", file.sha256sum());
	/// 	Ok(())
	/// }
	/// ```
	fn sha256sum(&mut self) -> Result<String>;

	/// this method returns the sha384-digest for implemented types as a [std::io::Result]
	/// of [String].
	/// # Example
	/// ```rust
	/// extern crate phollaits;
	/// use phollaits::HashExt;
	/// use std::fs;
	/// use std::io;
	/// fn main() -> io::Result<()> {
	/// 	let file = fs::File::open("/home/ph0llux/Pictures/image01.png")?;
	/// 	println!("{:?}", file.sha384sum());
	/// 	Ok(())
	/// }
	/// ```
	fn sha384sum(&mut self) -> Result<String>;

	/// this method returns the sha512-digest for implemented types as a [std::io::Result]
	/// of [String].
	/// # Example
	/// ```rust
	/// extern crate phollaits;
	/// use phollaits::HashExt;
	/// use std::fs;
	/// use std::io;
	/// fn main() -> io::Result<()> {
	/// 	let file = fs::File::open("/home/ph0llux/Pictures/image01.png")?;
	/// 	println!("{:?}", file.sha512sum());
	/// 	Ok(())
	/// }
	/// ```
	fn sha512sum(&mut self) -> Result<String>;
}

impl HashExt for dyn io::Read {
	fn md5sum(&mut self) -> Result<String> {
		use md5::Digest;
		let mut hasher = Md5::new();
		let mut buffer = [0; 1024];
		loop {
			let count = match self.read(&mut buffer){
				Ok(x) => x,
				Err(e) => return Err(PhollaitsError::new(
					PhollaitsErrorKind::HashingError,
					format!("Error while trying to hash input; {}", e.to_string())))
			};
			if count == 0 {
				break;
			}
			hasher.update(&buffer[..count]);
		}
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}

	fn sha1sum(&mut self) -> Result<String> {
		use sha1::Digest;
		let mut hasher = Sha1::new();
		let mut buffer = [0; 1024];
		loop {
			let count = match self.read(&mut buffer){
				Ok(x) => x,
				Err(e) => return Err(PhollaitsError::new(
					PhollaitsErrorKind::HashingError,
					format!("Error while trying to hash input; {}", e.to_string())))
			};
			if count == 0 {
				break;
			}
			hasher.update(&buffer[..count]);
		}
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}

	fn sha256sum(&mut self) -> Result<String> {
		use sha2::Digest;
		let mut hasher = Sha256::new();
		let mut buffer = [0; 1024];
		loop {
			let count = match self.read(&mut buffer){
				Ok(x) => x,
				Err(e) => return Err(PhollaitsError::new(
					PhollaitsErrorKind::HashingError,
					format!("Error while trying to hash input; {}", e.to_string())))
			};
			if count == 0 {
				break;
			}
			hasher.update(&buffer[..count]);
		}
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}

	fn sha384sum(&mut self) -> Result<String> {
		use sha2::Digest;
		let mut hasher = Sha384::new();
		let mut buffer = [0; 1024];
		loop {
			let count = match self.read(&mut buffer){
				Ok(x) => x,
				Err(e) => return Err(PhollaitsError::new(
					PhollaitsErrorKind::HashingError,
					format!("Error while trying to hash input; {}", e.to_string())))
			};
			if count == 0 {
				break;
			}
			hasher.update(&buffer[..count]);
		}
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}

	fn sha512sum(&mut self) -> Result<String> {
		use sha2::Digest;
		let mut hasher = Sha512::new();
		let mut buffer = [0; 1024];
		loop {
			let count = match self.read(&mut buffer){
				Ok(x) => x,
				Err(e) => return Err(PhollaitsError::new(
					PhollaitsErrorKind::HashingError,
					format!("Error while trying to hash input; {}", e.to_string())))
			};
			if count == 0 {
				break;
			}
			hasher.update(&buffer[..count]);
		}
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}
}

impl HashExt for std::fs::File {
	fn md5sum(&mut self) -> Result<String> {
		use md5::Digest;
		let mut hasher = Md5::new();
		let mut buffer = [0; 1024];
		loop {
			let count = match self.read(&mut buffer){
				Ok(x) => x,
				Err(e) => return Err(PhollaitsError::new(
					PhollaitsErrorKind::HashingError,
					format!("Error while trying to hash input; {}", e.to_string())))
			};
			if count == 0 {
				break;
			}
			hasher.update(&buffer[..count]);
		}
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}

	fn sha1sum(&mut self) -> Result<String> {
		use sha1::Digest;
		let mut hasher = Sha1::new();
		let mut buffer = [0; 1024];
		loop {
			let count = match self.read(&mut buffer){
				Ok(x) => x,
				Err(e) => return Err(PhollaitsError::new(
					PhollaitsErrorKind::HashingError,
					format!("Error while trying to hash input; {}", e.to_string())))
			};
			if count == 0 {
				break;
			}
			hasher.update(&buffer[..count]);
		}
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}

	fn sha256sum(&mut self) -> Result<String> {
		use sha2::Digest;
		let mut hasher = Sha256::new();
		let mut buffer = [0; 1024];
		loop {
			let count = match self.read(&mut buffer){
				Ok(x) => x,
				Err(e) => return Err(PhollaitsError::new(
					PhollaitsErrorKind::HashingError,
					format!("Error while trying to hash input; {}", e.to_string())))
			};
			if count == 0 {
				break;
			}
			hasher.update(&buffer[..count]);
		}
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}

	fn sha384sum(&mut self) -> Result<String> {
		use sha2::Digest;
		let mut hasher = Sha384::new();
		let mut buffer = [0; 1024];
		loop {
			let count = match self.read(&mut buffer){
				Ok(x) => x,
				Err(e) => return Err(PhollaitsError::new(
					PhollaitsErrorKind::HashingError,
					format!("Error while trying to hash input; {}", e.to_string())))
			};
			if count == 0 {
				break;
			}
			hasher.update(&buffer[..count]);
		}
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}

	fn sha512sum(&mut self) -> Result<String> {
		use sha2::Digest;
		let mut hasher = Sha512::new();
		let mut buffer = [0; 1024];
		loop {
			let count = match self.read(&mut buffer){
				Ok(x) => x,
				Err(e) => return Err(PhollaitsError::new(
					PhollaitsErrorKind::HashingError,
					format!("Error while trying to hash input; {}", e.to_string())))
			};
			if count == 0 {
				break;
			}
			hasher.update(&buffer[..count]);
		}
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}
}

impl<R: io::Read> HashExt for Entry<'_, R> {
	fn md5sum(&mut self) -> Result<String> {
		use md5::Digest;
		let mut hasher = Md5::new();
		let mut buffer = [0; 1024];
		loop {
			let count = match self.read(&mut buffer){
				Ok(x) => x,
				Err(e) => return Err(PhollaitsError::new(
					PhollaitsErrorKind::HashingError,
					format!("Error while trying to hash input; {}", e.to_string())))
			};
			if count == 0 {
				break;
			}
			hasher.update(&buffer[..count]);
		}
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}

	fn sha1sum(&mut self) -> Result<String> {
		use sha1::Digest;
		let mut hasher = Sha1::new();
		let mut buffer = [0; 1024];
		loop {
			let count = match self.read(&mut buffer){
				Ok(x) => x,
				Err(e) => return Err(PhollaitsError::new(
					PhollaitsErrorKind::HashingError,
					format!("Error while trying to hash input; {}", e.to_string())))
			};
			if count == 0 {
				break;
			}
			hasher.update(&buffer[..count]);
		}
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}

	fn sha256sum(&mut self) -> Result<String> {
		use sha2::Digest;
		let mut hasher = Sha256::new();
		let mut buffer = [0; 1024];
		loop {
			let count = match self.read(&mut buffer){
				Ok(x) => x,
				Err(e) => return Err(PhollaitsError::new(
					PhollaitsErrorKind::HashingError,
					format!("Error while trying to hash input; {}", e.to_string())))
			};
			if count == 0 {
				break;
			}
			hasher.update(&buffer[..count]);
		}
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}

	fn sha384sum(&mut self) -> Result<String> {
		use sha2::Digest;
		let mut hasher = Sha384::new();
		let mut buffer = [0; 1024];
		loop {
			let count = match self.read(&mut buffer){
				Ok(x) => x,
				Err(e) => return Err(PhollaitsError::new(
					PhollaitsErrorKind::HashingError,
					format!("Error while trying to hash input; {}", e.to_string())))
			};
			if count == 0 {
				break;
			}
			hasher.update(&buffer[..count]);
		}
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}

	fn sha512sum(&mut self) -> Result<String> {
		use sha2::Digest;
		let mut hasher = Sha512::new();
		let mut buffer = [0; 1024];
		loop {
			let count = match self.read(&mut buffer){
				Ok(x) => x,
				Err(e) => return Err(PhollaitsError::new(
					PhollaitsErrorKind::HashingError,
					format!("Error while trying to hash input; {}", e.to_string())))
			};
			if count == 0 {
				break;
			}
			hasher.update(&buffer[..count]);
		}
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}
}

impl HashExt for String {
	fn md5sum(&mut self) -> Result<String> {
		use md5::Digest;
		let mut hasher = Md5::new();
		hasher.update(self.as_bytes());
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}

	fn sha1sum(&mut self) -> Result<String> {
		use sha1::Digest;
		let mut hasher = Sha1::new();
		hasher.update(self.as_bytes());
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}

	fn sha256sum(&mut self) -> Result<String> {
		use sha2::Digest;
		let mut hasher = Sha256::new();
		hasher.update(self.as_bytes());
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}

	fn sha384sum(&mut self) -> Result<String> {
		use sha2::Digest;
		let mut hasher = Sha384::new();
		hasher.update(self.as_bytes());
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}

	fn sha512sum(&mut self) -> Result<String> {
		use sha2::Digest;
		let mut hasher = Sha512::new();
		hasher.update(self.as_bytes());
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}
}

impl HashExt for u64 {
	fn md5sum(&mut self) -> Result<String> {
		use md5::Digest;
		let mut hasher = Md5::new();
		hasher.update(self.to_be_bytes());
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}

	fn sha1sum(&mut self) -> Result<String> {
		use sha1::Digest;
		let mut hasher = Sha1::new();
		hasher.update(self.to_be_bytes());
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}

	fn sha256sum(&mut self) -> Result<String> {
		use sha2::Digest;
		let mut hasher = Sha256::new();
		hasher.update(self.to_be_bytes());
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}

	fn sha384sum(&mut self) -> Result<String> {
		use sha2::Digest;
		let mut hasher = Sha384::new();
		hasher.update(self.to_be_bytes());
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}

	fn sha512sum(&mut self) -> Result<String> {
		use sha2::Digest;
		let mut hasher = Sha512::new();
		hasher.update(self.to_be_bytes());
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}
}