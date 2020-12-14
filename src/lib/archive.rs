/************************************************************************
* pk:45a53fa8ec330c1e0977d9ba821f200c89b789bd07a1a2aac5dffe1787b191c5
************************************************************************/
//!tar Module.

// 
// - STD
use std::fs::File;
use std::io;
use std::path::Path;

// 
// - external
use tar::{Builder, Header};

/// Trait implements some extensions for the [Builder](https://docs.rs/tar/0.4.30/tar/struct.Builder.html)-struct of the [tar](https://docs.rs/tar/0.4.30/tar/) crate.
pub trait TarBuilderExt {
	/// appends a file to an archive.
	/// # Example
	/// ```rust
	/// extern crate tar;
	/// extern crate phollaits;
	///
	/// use phollaits::*;
	/// use tar::Builder;
	///
	/// fn main() {
	/// 	let b = Builder::new("/tmp/archive.tar");
	/// 	b.append_file("/home/ph0llux/example01.png"); //appends a file (absoulte path)
	/// 	b.append("example02.png"); //appends a file (relative path)
	/// 	b.close_archive();
	/// }
	/// ```
	fn append_file<P: Into<String>>(&mut self, path: P);

	/// appends a text (string) to an archive.
	/// # Example
	/// ```rust
	/// extern crate tar;
	/// extern crate phollaits;
	///
	/// use phollaits::*;
	/// use tar::Builder;
	///
	/// fn main() {
	/// 	let b = Builder::new("/tmp/archive.tar");
	/// 	let content = "this is an example text";
	/// 	let filename_in_archive = "/home/ph0llux/example01.txt";
	/// 	b.append_text(filename_in_archive, content);
	/// 	b.close_archive();
	/// }
	/// ```
	fn append_text<F: Into<String>, T: Into<String>>(&mut self, filename: F, text: T) -> io::Result<()>;

	/// This method simply calls the [into_inner()](https://docs.rs/tar/0.4.30/tar/struct.Builder.html#method.into_inner)
	/// method. This method is used solely for embellishment purposes.
	/// you can call the [into_inner()](https://docs.rs/tar/0.4.30/tar/struct.Builder.html#method.into_inner)
	/// method directly.
	fn close_archive(self) -> io::Result<File>;
}

impl TarBuilderExt for Builder<File> {
	fn append_file<P: Into<String>>(&mut self, path: P) {
		let path = path.into();
		if Path::new(&path).is_absolute() {
			&self.append_path_with_name(&path, &path[1..])
		} else {
			&self.append_path(path)
		};
	}
	fn append_text<F: Into<String>, T: Into<String>>(&mut self, filename: F, text: T) -> io::Result<()> {
		let filename = filename.into();
		let text = text.into();
		let mut header = Header::new_gnu();
		header.set_size(text.len() as u64); //header size must be == size of string in bytes.
		header.set_cksum();
		self.append_data(&mut header, &filename, text.as_bytes())
	}
	fn close_archive(self) -> io::Result<File> {
		self.into_inner()
	}
}
