/*************************************************************************
* ph0llux:924ed9e854e417a2c055135d3d0a019d9dc3c1863b67640ca8446581845dcd82
*************************************************************************/
//!tar Module.

// - STD
use std::fs::File;
use std::time::SystemTime;
use std::io;
use std::path::Path;

// - external
use tar::{Builder,Header,EntryType};

// - internal
use super::{PhollaitsError, PhollaitsErrorKind, Result};

/// Trait implements some extensions for the [Builder](https://docs.rs/tar/0.4.30/tar/struct.Builder.html)-struct of the [tar](https://docs.rs/tar/0.4.30/tar/) crate.
pub trait TarBuilderExt {
	/// appends a file to an archive.
	/// # Example
	/// ```
	/// extern crate tar;
	/// extern crate phollaits;
	///
	/// use phollaits::*;
	/// use tar::Builder;
	///	use std::fs;
	///
	/// fn main() {
	///		let mut file = fs::File::create("/tmp/archive.tar").unwrap();
	/// 	let mut b = Builder::new(file);
	/// 	//b.append_file_directly("/etc/os-release"); //appends a file (absoulte path)
	/// 	b.append_file_directly("assets/example.jpg"); //appends a file (relative path)
	/// 	b.close_archive();
	/// }
	/// ```
	fn append_file_directly<P: Into<String>>(&mut self, path: P) -> Result<()>;

	/// appends a text (string) to an archive. The text will be written as a textfile, with the "unix-like" file permissions 644.
	/// # Example
	/// ```
	/// extern crate tar;
	/// extern crate phollaits;
	///
	/// use phollaits::*;
	/// use tar::Builder;
	/// use std::fs;
	///
	/// fn main() {
	///		let mut file = fs::File::create("/tmp/archive.tar").unwrap();
	/// 	let mut b = Builder::new(file);
	/// 	let content = "this is an example text";
	/// 	let filename_in_archive = "/home/ph0llux/example01.txt";
	/// 	b.append_text(filename_in_archive, content);
	/// 	b.close_archive();
	/// }
	/// ```
	fn append_text<F: Into<String>, T: Into<String>>(&mut self, filename: F, text: T) -> Result<()>;

	/// This method simply calls the [into_inner()](https://docs.rs/tar/0.4.30/tar/struct.Builder.html#method.into_inner)
	/// method. This method is used solely for embellishment purposes.
	/// you can call the [into_inner()](https://docs.rs/tar/0.4.30/tar/struct.Builder.html#method.into_inner)
	/// method directly.
	fn close_archive(self) -> Result<()>;
}

impl TarBuilderExt for Builder<File> {
	fn append_file_directly<P: Into<String>>(&mut self, path: P) -> Result<()> {
		let path = path.into();
		if Path::new(&path).is_absolute() {
			match self.append_path_with_name(&path, &path[1..]) {
				Ok(x) => return Ok(x),
				Err(e) => return Err(PhollaitsError::new(
					PhollaitsErrorKind::ArchiveError,
					format!("Error while trying to append file directly (append_path_with_name); {}", e.to_string())))
			};
		} else {
			match self.append_path(path) {
				Ok(x) => return Ok(x),
				Err(e) => return Err(PhollaitsError::new(
					PhollaitsErrorKind::ArchiveError,
					format!("Error while trying to append file directly (append_path); {}", e.to_string())))
			};
		};
	}
	fn append_text<F: Into<String>, T: Into<String>>(&mut self, filename: F, text: T) -> Result<()> {
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
		match self.append_data(&mut header, &filename, text.as_bytes()) {
				Ok(x) => return Ok(x),
				Err(e) => return Err(PhollaitsError::new(
					PhollaitsErrorKind::ArchiveError,
					format!("Error while trying to append text; {}", e.to_string())))
			};
	}
	fn close_archive(mut self) -> Result<()> {
		match self.finish() {
			Ok(x) => return Ok(x),
			Err(e) => return Err(PhollaitsError::new(
				PhollaitsErrorKind::ArchiveError,
				format!("Error while trying to append text; {}", e.to_string())))
		};
	}
}

impl TarBuilderExt for Builder<Box<dyn io::Write>> {
	fn append_file_directly<P: Into<String>>(&mut self, path: P) -> Result<()> {
		let path = path.into();
		if Path::new(&path).is_absolute() {
			match self.append_path_with_name(&path, &path[1..]) {
				Ok(x) => return Ok(x),
				Err(e) => return Err(PhollaitsError::new(
					PhollaitsErrorKind::ArchiveError,
					format!("Error while trying to append file directly (append_path_with_name); {}", e.to_string())))
			};
		} else {
			match self.append_path(path) {
				Ok(x) => return Ok(x),
				Err(e) => return Err(PhollaitsError::new(
					PhollaitsErrorKind::ArchiveError,
					format!("Error while trying to append file directly (append_path); {}", e.to_string())))
			};
		};
	}
	fn append_text<F: Into<String>, T: Into<String>>(&mut self, filename: F, text: T) -> Result<()> {
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
		match self.append_data(&mut header, &filename, text.as_bytes()) {
				Ok(x) => return Ok(x),
				Err(e) => return Err(PhollaitsError::new(
					PhollaitsErrorKind::ArchiveError,
					format!("Error while trying to append text; {}", e.to_string())))
			};
	}
	fn close_archive(mut self) -> Result<()> {
		match self.finish() {
			Ok(x) => return Ok(x),
			Err(e) => return Err(PhollaitsError::new(
				PhollaitsErrorKind::ArchiveError,
				format!("Error while trying to append text; {}", e.to_string())))
		};
	}
}