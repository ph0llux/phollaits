/*************************************************************************
* ph0llux:b672bbb1f54d525d95818ee050f0ad73088e9bd31fe7c07ed4225335c01e42e3
*************************************************************************/
// - STD
use std::num::ParseIntError;
use std::fmt;

pub struct PhollaitsError {
	details: String,
	kind: PhollaitsErrorKind
}

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