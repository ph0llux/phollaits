/************************************************************************
* pk:4a3442ad21eb564167b451ce148286a3b0bec591e9e634165b94516df4cec648
************************************************************************/
//!hash Module.

// 
// - STD
use std::io;
use std::io::Read;

// 
// - external
use data_encoding::HEXUPPER;
use md5::Context as Md5Context;
use ring::digest::Context as ShaContext;
use ring::digest::{SHA1_FOR_LEGACY_USE_ONLY, SHA256, SHA384, SHA512};
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
	fn md5sum(&mut self) -> io::Result<String>;

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
	fn sha1sum(&mut self) -> io::Result<String>;

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
	fn sha256sum(&mut self) -> io::Result<String>;

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
	fn sha384sum(&mut self) -> io::Result<String>;

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
	fn sha512sum(&mut self) -> io::Result<String>;
}

impl HashExt for dyn io::Read {
	fn md5sum(&mut self) -> io::Result<String> {
		let mut context = Md5Context::new();
		let mut buffer = [0; 1024];
		loop {
			let count = self.read(&mut buffer)?;
			if count == 0 {
				break;
			}
			context.consume(&buffer[..count]);
		}
		let context = context.compute();
		Ok(HEXUPPER.encode(context.as_ref()))
	}

	fn sha1sum(&mut self) -> io::Result<String> {
		let mut context = ShaContext::new(&SHA1_FOR_LEGACY_USE_ONLY);
		let mut buffer = [0; 1024];
		loop {
			let count = self.read(&mut buffer)?;
			if count == 0 {
				break;
			}
			context.update(&buffer[..count]);
		}
		let context = context.finish();
		Ok(HEXUPPER.encode(context.as_ref()))
	}

	fn sha256sum(&mut self) -> io::Result<String> {
		let mut context = ShaContext::new(&SHA256);
		let mut buffer = [0; 1024];
		loop {
			let count = self.read(&mut buffer)?;
			if count == 0 {
				break;
			}
			context.update(&buffer[..count]);
		}
		let context = context.finish();
		Ok(HEXUPPER.encode(context.as_ref()))
	}

	fn sha384sum(&mut self) -> io::Result<String> {
		let mut context = ShaContext::new(&SHA384);
		let mut buffer = [0; 1024];
		loop {
			let count = self.read(&mut buffer)?;
			if count == 0 {
				break;
			}
			context.update(&buffer[..count]);
		}
		let context = context.finish();
		Ok(HEXUPPER.encode(context.as_ref()))
	}

	fn sha512sum(&mut self) -> io::Result<String> {
		let mut context = ShaContext::new(&SHA512);
		let mut buffer = [0; 1024];
		loop {
			let count = self.read(&mut buffer)?;
			if count == 0 {
				break;
			}
			context.update(&buffer[..count]);
		}
		let context = context.finish();
		Ok(HEXUPPER.encode(context.as_ref()))
	}
}

impl HashExt for std::fs::File {
	fn md5sum(&mut self) -> io::Result<String> {
		let mut context = Md5Context::new();
		let mut buffer = [0; 1024];
		loop {
			let count = self.read(&mut buffer)?;
			if count == 0 {
				break;
			}
			context.consume(&buffer[..count]);
		}
		let context = context.compute();
		Ok(HEXUPPER.encode(context.as_ref()))
	}

	fn sha1sum(&mut self) -> io::Result<String> {
		let mut context = ShaContext::new(&SHA1_FOR_LEGACY_USE_ONLY);
		let mut buffer = [0; 1024];
		loop {
			let count = self.read(&mut buffer)?;
			if count == 0 {
				break;
			}
			context.update(&buffer[..count]);
		}
		let context = context.finish();
		Ok(HEXUPPER.encode(context.as_ref()))
	}

	fn sha256sum(&mut self) -> io::Result<String> {
		let mut context = ShaContext::new(&SHA256);
		let mut buffer = [0; 1024];
		loop {
			let count = self.read(&mut buffer)?;
			if count == 0 {
				break;
			}
			context.update(&buffer[..count]);
		}
		let context = context.finish();
		Ok(HEXUPPER.encode(context.as_ref()))
	}

	fn sha384sum(&mut self) -> io::Result<String> {
		let mut context = ShaContext::new(&SHA384);
		let mut buffer = [0; 1024];
		loop {
			let count = self.read(&mut buffer)?;
			if count == 0 {
				break;
			}
			context.update(&buffer[..count]);
		}
		let context = context.finish();
		Ok(HEXUPPER.encode(context.as_ref()))
	}

	fn sha512sum(&mut self) -> io::Result<String> {
		let mut context = ShaContext::new(&SHA512);
		let mut buffer = [0; 1024];
		loop {
			let count = self.read(&mut buffer)?;
			if count == 0 {
				break;
			}
			context.update(&buffer[..count]);
		}
		let context = context.finish();
		Ok(HEXUPPER.encode(context.as_ref()))
	}
}

impl<R: io::Read> HashExt for Entry<'_, R> {
	fn md5sum(&mut self) -> io::Result<String> {
		let mut context = Md5Context::new();
		let mut buffer = [0; 1024];
		loop {
			let count = self.read(&mut buffer)?;
			if count == 0 {
				break;
			}
			context.consume(&buffer[..count]);
		}
		let context = context.compute();
		Ok(HEXUPPER.encode(context.as_ref()))
	}

	fn sha1sum(&mut self) -> io::Result<String> {
		let mut context = ShaContext::new(&SHA1_FOR_LEGACY_USE_ONLY);
		let mut buffer = [0; 1024];
		loop {
			let count = self.read(&mut buffer)?;
			if count == 0 {
				break;
			}
			context.update(&buffer[..count]);
		}
		let context = context.finish();
		Ok(HEXUPPER.encode(context.as_ref()))
	}

	fn sha256sum(&mut self) -> io::Result<String> {
		let mut context = ShaContext::new(&SHA256);
		let mut buffer = [0; 1024];
		loop {
			let count = self.read(&mut buffer)?;
			if count == 0 {
				break;
			}
			context.update(&buffer[..count]);
		}
		let context = context.finish();
		Ok(HEXUPPER.encode(context.as_ref()))
	}

	fn sha384sum(&mut self) -> io::Result<String> {
		let mut context = ShaContext::new(&SHA384);
		let mut buffer = [0; 1024];
		loop {
			let count = self.read(&mut buffer)?;
			if count == 0 {
				break;
			}
			context.update(&buffer[..count]);
		}
		let context = context.finish();
		Ok(HEXUPPER.encode(context.as_ref()))
	}

	fn sha512sum(&mut self) -> io::Result<String> {
		let mut context = ShaContext::new(&SHA512);
		let mut buffer = [0; 1024];
		loop {
			let count = self.read(&mut buffer)?;
			if count == 0 {
				break;
			}
			context.update(&buffer[..count]);
		}
		let context = context.finish();
		Ok(HEXUPPER.encode(context.as_ref()))
	}
}

impl HashExt for String {
	fn md5sum(&mut self) -> io::Result<String> {
		let mut context = Md5Context::new();
		context.consume(self.as_bytes());
		let context = context.compute();
		Ok(HEXUPPER.encode(context.as_ref()))
	}

	fn sha1sum(&mut self) -> io::Result<String> {
		let mut context = ShaContext::new(&SHA1_FOR_LEGACY_USE_ONLY);
		context.update(self.as_bytes());
		let context = context.finish();
		Ok(HEXUPPER.encode(context.as_ref()))
	}

	fn sha256sum(&mut self) -> io::Result<String> {
		let mut context = ShaContext::new(&SHA256);
		context.update(self.as_bytes());
		let context = context.finish();
		Ok(HEXUPPER.encode(context.as_ref()))
	}

	fn sha384sum(&mut self) -> io::Result<String> {
		let mut context = ShaContext::new(&SHA384);
		context.update(self.as_bytes());
		let context = context.finish();
		Ok(HEXUPPER.encode(context.as_ref()))
	}

	fn sha512sum(&mut self) -> io::Result<String> {
		let mut context = ShaContext::new(&SHA512);
		context.update(self.as_bytes());
		let context = context.finish();
		Ok(HEXUPPER.encode(context.as_ref()))
	}
}

impl HashExt for u64 {
	fn md5sum(&mut self) -> io::Result<String> {
		let mut context = Md5Context::new();
		context.consume(self.to_be_bytes());
		let context = context.compute();
		Ok(HEXUPPER.encode(context.as_ref()))
	}

	fn sha1sum(&mut self) -> io::Result<String> {
		let mut context = ShaContext::new(&SHA1_FOR_LEGACY_USE_ONLY);
		context.update(&self.to_be_bytes());
		let context = context.finish();
		Ok(HEXUPPER.encode(context.as_ref()))
	}

	fn sha256sum(&mut self) -> io::Result<String> {
		let mut context = ShaContext::new(&SHA256);
		context.update(&self.to_be_bytes());
		let context = context.finish();
		Ok(HEXUPPER.encode(context.as_ref()))
	}

	fn sha384sum(&mut self) -> io::Result<String> {
		let mut context = ShaContext::new(&SHA384);
		context.update(&self.to_be_bytes());
		let context = context.finish();
		Ok(HEXUPPER.encode(context.as_ref()))
	}

	fn sha512sum(&mut self) -> io::Result<String> {
		let mut context = ShaContext::new(&SHA512);
		context.update(&self.to_be_bytes());
		let context = context.finish();
		Ok(HEXUPPER.encode(context.as_ref()))
	}
}