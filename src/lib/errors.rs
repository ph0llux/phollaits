/*************************************************************************
* ph0llux:12814db628416164b3aa4728c497221589d1eac4cca7184a696fd0255b2df1cd
*************************************************************************/
// - STD
use std::num::ParseIntError;
use std::fmt;

#[derive(Debug)]
pub struct PhollaitsError {
	details: String,
	kind: PhollaitsErrorKind
}

#[derive(Debug)]
pub enum PhollaitsErrorKind {
	ParseIntError,
	HashingError,
	ArchiveError,
}

impl PhollaitsError {
	pub fn new<S: Into<String>>(kind: PhollaitsErrorKind, details: S) -> PhollaitsError {
		let details = details.into();
		PhollaitsError {
			details: details,
			kind: kind
		}
	}
}

impl fmt::Display for PhollaitsErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let err_msg = match self {
			PhollaitsErrorKind::ParseIntError => "ParseIntError",
			PhollaitsErrorKind::HashingError => "HashingError",
			PhollaitsErrorKind::ArchiveError => "ArchiveError",
		};
	write!(f, "{}", err_msg)
	}
}

impl From<ParseIntError> for PhollaitsError {
	fn from(e: ParseIntError) -> PhollaitsError {
		PhollaitsError::new(PhollaitsErrorKind::ParseIntError, e.to_string())
	}
}

impl fmt::Display for PhollaitsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let err_msg = format!("{}: {}", self.kind, self.details);
	write!(f, "{}", err_msg)
	}
}