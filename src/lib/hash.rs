/*************************************************************************
* ph0llux:61b17685413cf4b155779f471486c30048fd52a90925d481ea777267748130ff
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
	/// ```
	/// extern crate phollaits;
	/// use phollaits::{HashExt, Result};
	/// use std::fs;
	/// use std::io;
	/// 
	/// fn main() -> Result<()> {
	/// 	// usage with File types
	/// 	let mut file = fs::File::open(FILENAME_01).unwrap();
	/// 	assert_eq!(file.md5sum()?, String::from(FILE01_HASH));
	/// 
	/// 	// usage with dyn io::Read
	/// 	let mut io_read = file_as_io_read(FILENAME_01).unwrap();
	/// 	assert_eq!(io_read.md5sum()?, String::from(FILE01_HASH));
	/// 
	/// 	// usage with String
	/// 	let mut s = fs::read_to_string(FILENAME_02).unwrap();
	///		assert_eq!(s.md5sum()?, String::from(FILE02_HASH));
	/// 
	/// 	Ok(())
	/// }
	/// 
	/// fn file_as_io_read<I: Into<String>>(filename: I) -> io::Result<Box<dyn io::Read>>
	/// {
	/// 	let input = fs::File::open(filename.into())?;
	/// 	Ok(Box::new(io::BufReader::new(input)))	
	/// }
	/// 
	/// const FILENAME_01: &'static str = "assets/example.jpg";
	///	const FILENAME_02: &'static str = "assets/textfile.txt";
	/// const FILE01_HASH: &'static str = "a4494bd1b83303bc0872a996e6c8a8bf";
	/// const FILE02_HASH: &'static str = "25f9e794323b453885f5181f1b624d0b";
	/// ```
	fn md5sum(&mut self) -> Result<String>;

	/// this method returns the sha1-digest for implemented types as a [std::io::Result]
	/// of [String].
	/// # Example
	/// ```
	/// extern crate phollaits;
	/// use phollaits::{HashExt, Result};
	/// use std::fs;
	/// use std::io;
	/// 
	/// fn main() -> Result<()> {
	/// 	// usage with File types
	/// 	let mut file = fs::File::open(FILENAME_01).unwrap();
	/// 	assert_eq!(file.sha1sum()?, String::from(FILE01_HASH));
	/// 
	/// 	// usage with dyn io::Read
	/// 	let mut io_read = file_as_io_read(FILENAME_01).unwrap();
	/// 	assert_eq!(io_read.sha1sum()?, String::from(FILE01_HASH));
	/// 
	/// 	// usage with String
	/// 	let mut s = fs::read_to_string(FILENAME_02).unwrap();
	///		assert_eq!(s.sha1sum()?, String::from(FILE02_HASH));
	/// 
	/// 	Ok(())
	/// }
	/// 
	/// fn file_as_io_read<I: Into<String>>(filename: I) -> io::Result<Box<dyn io::Read>>
	/// {
	/// 	let input = fs::File::open(filename.into())?;
	/// 	Ok(Box::new(io::BufReader::new(input)))	
	/// }
	/// 
	/// const FILENAME_01: &'static str = "assets/example.jpg";
	///	const FILENAME_02: &'static str = "assets/textfile.txt";
	/// const FILE01_HASH: &'static str = "fc01e2d6df94d08cdf825ca0dc87a042ae73ba26";
	/// const FILE02_HASH: &'static str = "f7c3bc1d808e04732adf679965ccc34ca7ae3441";
	/// ```
	fn sha1sum(&mut self) -> Result<String>;

	/// this method returns the sha256-digest for implemented types as a [std::io::Result]
	/// of [String].
	/// # Example
	/// ```
	/// extern crate phollaits;
	/// use phollaits::{HashExt, Result};
	/// use std::fs;
	/// use std::io;
	/// 
	/// fn main() -> Result<()> {
	/// 	// usage with File types
	/// 	let mut file = fs::File::open(FILENAME_01).unwrap();
	/// 	assert_eq!(file.sha256sum()?, String::from(FILE01_HASH));
	/// 
	/// 	// usage with dyn io::Read
	/// 	let mut io_read = file_as_io_read(FILENAME_01).unwrap();
	/// 	assert_eq!(io_read.sha256sum()?, String::from(FILE01_HASH));
	/// 
	/// 	// usage with String
	/// 	let mut s = fs::read_to_string(FILENAME_02).unwrap();
	///		assert_eq!(s.sha256sum()?, String::from(FILE02_HASH));
	/// 
	/// 	Ok(())
	/// }
	/// 
	/// fn file_as_io_read<I: Into<String>>(filename: I) -> io::Result<Box<dyn io::Read>>
	/// {
	/// 	let input = fs::File::open(filename.into())?;
	/// 	Ok(Box::new(io::BufReader::new(input)))	
	/// }
	/// 
	/// const FILENAME_01: &'static str = "assets/example.jpg";
	///	const FILENAME_02: &'static str = "assets/textfile.txt";
	/// const FILE01_HASH: &'static str = "5b123b99225b5117bb7553929db40e8536bf84e687629bde05184cccbb734793";
	/// const FILE02_HASH: &'static str = "15e2b0d3c33891ebb0f1ef609ec419420c20e320ce94c65fbc8c3312448eb225";
	/// ```
	fn sha256sum(&mut self) -> Result<String>;

	/// this method returns the sha384-digest for implemented types as a [std::io::Result]
	/// of [String].
	/// # Example
	/// ```
	/// extern crate phollaits;
	/// use phollaits::{HashExt, Result};
	/// use std::fs;
	/// use std::io;
	/// 
	/// fn main() -> Result<()> {
	/// 	// usage with File types
	/// 	let mut file = fs::File::open(FILENAME_01).unwrap();
	/// 	assert_eq!(file.sha384sum()?, String::from(FILE01_HASH));
	/// 
	/// 	// usage with dyn io::Read
	/// 	let mut io_read = file_as_io_read(FILENAME_01).unwrap();
	/// 	assert_eq!(io_read.sha384sum()?, String::from(FILE01_HASH));
	/// 
	/// 	// usage with String
	/// 	let mut s = fs::read_to_string(FILENAME_02).unwrap();
	///		assert_eq!(s.sha384sum()?, String::from(FILE02_HASH));
	/// 
	/// 	Ok(())
	/// }
	/// 
	/// fn file_as_io_read<I: Into<String>>(filename: I) -> io::Result<Box<dyn io::Read>>
	/// {
	/// 	let input = fs::File::open(filename.into())?;
	/// 	Ok(Box::new(io::BufReader::new(input)))	
	/// }
	/// 
	/// const FILENAME_01: &'static str = "assets/example.jpg";
	///	const FILENAME_02: &'static str = "assets/textfile.txt";
	/// const FILE01_HASH: &'static str = "ff21329fcdee214b3a99694a2c65492e62154850fdb8a92f2a9efbb1185acc1c416de06f248e5b058b94f0e87f876c52";
	/// const FILE02_HASH: &'static str = "eb455d56d2c1a69de64e832011f3393d45f3fa31d6842f21af92d2fe469c499da5e3179847334a18479c8d1dedea1be3";
	/// ```
	fn sha384sum(&mut self) -> Result<String>;

	/// this method returns the sha512-digest for implemented types as a [std::io::Result]
	/// of [String].
	/// # Example
	/// ```
	/// extern crate phollaits;
	/// use phollaits::{HashExt, Result};
	/// use std::fs;
	/// use std::io;
	/// 
	/// fn main() -> Result<()> {
	/// 	// usage with File types
	/// 	let mut file = fs::File::open(FILENAME_01).unwrap();
	/// 	assert_eq!(file.sha512sum()?, String::from(FILE01_HASH));
	/// 
	/// 	// usage with dyn io::Read
	/// 	let mut io_read = file_as_io_read(FILENAME_01).unwrap();
	/// 	assert_eq!(io_read.sha512sum()?, String::from(FILE01_HASH));
	/// 
	/// 	// usage with String
	/// 	let mut s = fs::read_to_string(FILENAME_02).unwrap();
	///		assert_eq!(s.sha512sum()?, String::from(FILE02_HASH));
	/// 
	/// 	Ok(())
	/// }
	/// 
	/// fn file_as_io_read<I: Into<String>>(filename: I) -> io::Result<Box<dyn io::Read>>
	/// {
	/// 	let input = fs::File::open(filename.into())?;
	/// 	Ok(Box::new(io::BufReader::new(input)))	
	/// }
	/// 
	/// const FILENAME_01: &'static str = "assets/example.jpg";
	///	const FILENAME_02: &'static str = "assets/textfile.txt";
	/// const FILE01_HASH: &'static str = "7b2889fbf27128ebf3459f8593c8c3562b4faaedb1a9eaf49129c0fc6ec880e86e22428f906d19f208cc4176ab6bc5d2875065dc4a27fb19db562a6e7cd10593";
	/// const FILE02_HASH: &'static str = "d9e6762dd1c8eaf6d61b3c6192fc408d4d6d5f1176d0c29169bc24e71c3f274ad27fcd5811b313d681f7e55ec02d73d499c95455b6b5bb503acf574fba8ffe85";
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
		hasher.update(self);
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}

	fn sha1sum(&mut self) -> Result<String> {
		use sha1::Digest;
		let mut hasher = Sha1::new();
		hasher.update(self);
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}

	fn sha256sum(&mut self) -> Result<String> {
		use sha2::Digest;
		let mut hasher = Sha256::new();
		hasher.update(self);
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}

	fn sha384sum(&mut self) -> Result<String> {
		use sha2::Digest;
		let mut hasher = Sha384::new();
		hasher.update(self);
		let hasher = hasher.finalize();
		Ok(HEXLOWER.encode(hasher.as_ref()))
	}

	fn sha512sum(&mut self) -> Result<String> {
		use sha2::Digest;
		let mut hasher = Sha512::new();
		hasher.update(self);
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