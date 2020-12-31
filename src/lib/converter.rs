/************************************************************************
* pk:abeed470db777162010c25b3c2d58a9cc32e3a52ef8196330f7587910b555040
************************************************************************/
//!converter module
pub trait HumanReadable {
	fn bytes_as_hrb(self) -> String;
}

impl<T: Into<f64>> HumanReadable for T {
	/// get bytes in a human readable format (returned as [String]). Can be applied on every type, which implements Into<f64>.
	/// # Example
	/// ```rust
	/// extern crate phollaits;
	/// use phollaits::*;
	///
	/// fn main() {
	/// 	let size = 2498566;
	/// 	assert_eq!(size.bytes_as_hrb(), "2.50MB");
	/// }
	/// ```
	fn bytes_as_hrb(self) -> String {
		const DIVISOR: f64 = 1000.0; //No, it's not 1024 - because we will calculate MB, not MiB. ;)
		const UNIT: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
		let mut current_multiplier = 1.0;
		let mut humanreadable_size = String::new();
		let size = self.into();
		while (size) >= DIVISOR.powf(current_multiplier) {
			current_multiplier += 1.0;
		}
		humanreadable_size
			.push_str(&(format!("{:.2}", (size) / DIVISOR.powf(current_multiplier - 1.0))).to_string());
		humanreadable_size.push_str(&(UNIT[(current_multiplier - 1.0) as usize].to_string()));
		humanreadable_size
	}
}


	
	
	
		