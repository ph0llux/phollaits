/************************************************************************
* pk:fc13689c6df8aacde27e0abb4814bf63ac4ac483334213de4d594d666663584e
************************************************************************/
//!tar Module.

// 
// - STD
use std::fs::File;
use std::time::SystemTime;
use std::io;
use std::path::Path;

// 
// - external
use tar::{Builder,Header,EntryType};

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
	/// 	b.append_file_directly("/home/ph0llux/example01.png"); //appends a file (absoulte path)
	/// 	b.append_file_directly("example02.png"); //appends a file (relative path)
	/// 	b.close_archive();
	/// }
	/// ```
	fn append_file_directly<P: Into<String>>(&mut self, path: P) -> io::Result<()>;

	/// appends a text (string) to an archive. The text will be written as a textfile, with the "unix-like" file permissions 644.
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
	fn close_archive(self) -> io::Result<()>;
}

impl TarBuilderExt for Builder<File> {
	fn append_file_directly<P: Into<String>>(&mut self, path: P) -> io::Result<()> {
		let path = path.into();
		if Path::new(&path).is_absolute() {
			return self.append_path_with_name(&path, &path[1..]);
		} else {
			return self.append_path(path);
		};
	}
	fn append_text<F: Into<String>, T: Into<String>>(&mut self, filename: F, text: T) -> io::Result<()> {
		let filename = filename.into();
		let text = text.into();
		let mut header = Header::new_gnu();
		header.set_size(text.len() as u64); //header size must be == size of string in bytes.
		header.set_cksum();
		header.set_mode(420); //the value will be converted into octal (420 -> 644, 40 -> 50, ...)
		header.set_entry_type(EntryType::file());
		match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) { //sets the current time as mtime for the file
			Ok(n) => header.set_mtime(n.as_secs()),
			Err(_) => ()
		};
		self.append_data(&mut header, &filename, text.as_bytes())
	}
	fn close_archive(mut self) -> io::Result<()> {
		self.finish()
	}
}

impl TarBuilderExt for Builder<Box<dyn io::Write>> {
	fn append_file_directly<P: Into<String>>(&mut self, path: P) -> io::Result<()> {
		let path = path.into();
		if Path::new(&path).is_absolute() {
			return self.append_path_with_name(&path, &path[1..]);
		} else {
			return self.append_path(path);
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
	fn close_archive(mut self) -> io::Result<()> {
		self.finish()
	}
}