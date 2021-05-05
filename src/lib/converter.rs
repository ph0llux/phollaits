/*************************************************************************
* ph0llux:267577322be553ec80d14acfc1429831fd963519246bc372dbd187cc1c7c4d0f
*************************************************************************/
//!converter module

pub trait HumanReadable {
	/// get bytes in a human readable format (returned as [String]). Can be applied on several numeric types.
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
	fn bytes_as_hrb(self) -> String;
}

impl HumanReadable for f32 {
	fn bytes_as_hrb(self) -> String {
		const DIVISOR: f32 = 1000.0; //No, it's not 1024 - because we will calculate MB, not MiB. ;)
		const UNIT: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
		let mut current_multiplier = 1.0;
		let mut humanreadable_size = String::new();
		let size = self as f32;
		while (size) >= DIVISOR.powf(current_multiplier) {
			current_multiplier += 1.0;
		}
		humanreadable_size
			.push_str(&(format!("{:.2}", (size) / DIVISOR.powf(current_multiplier - 1.0))).to_string());
		humanreadable_size.push_str(&(UNIT[(current_multiplier - 1.0) as usize].to_string()));
		humanreadable_size
	}
}

impl HumanReadable for f64 {
	fn bytes_as_hrb(self) -> String {
		const DIVISOR: f64 = 1000.0; //No, it's not 1024 - because we will calculate MB, not MiB. ;)
		const UNIT: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
		let mut current_multiplier = 1.0;
		let mut humanreadable_size = String::new();
		let size: f64 = self.into();
		while (size) >= DIVISOR.powf(current_multiplier) {
			current_multiplier += 1.0;
		}
		humanreadable_size
			.push_str(&(format!("{:.2}", (size) / DIVISOR.powf(current_multiplier - 1.0))).to_string());
		humanreadable_size.push_str(&(UNIT[(current_multiplier - 1.0) as usize].to_string()));
		humanreadable_size
	}
}

impl HumanReadable for i8 {
	fn bytes_as_hrb(self) -> String {
		const DIVISOR: f32 = 1000.0; //No, it's not 1024 - because we will calculate MB, not MiB. ;)
		const UNIT: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
		let mut current_multiplier = 1.0;
		let mut humanreadable_size = String::new();
		let size = self as f32;
		while (size) >= DIVISOR.powf(current_multiplier) {
			current_multiplier += 1.0;
		}
		humanreadable_size
			.push_str(&(format!("{:.2}", (size) / DIVISOR.powf(current_multiplier - 1.0))).to_string());
		humanreadable_size.push_str(&(UNIT[(current_multiplier - 1.0) as usize].to_string()));
		humanreadable_size
	}
}

impl HumanReadable for i16 {
	fn bytes_as_hrb(self) -> String {
		const DIVISOR: f32 = 1000.0; //No, it's not 1024 - because we will calculate MB, not MiB. ;)
		const UNIT: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
		let mut current_multiplier = 1.0;
		let mut humanreadable_size = String::new();
		let size = self as f32;
		while (size) >= DIVISOR.powf(current_multiplier) {
			current_multiplier += 1.0;
		}
		humanreadable_size
			.push_str(&(format!("{:.2}", (size) / DIVISOR.powf(current_multiplier - 1.0))).to_string());
		humanreadable_size.push_str(&(UNIT[(current_multiplier - 1.0) as usize].to_string()));
		humanreadable_size
	}
}

impl HumanReadable for i32 {
	fn bytes_as_hrb(self) -> String {
		const DIVISOR: f64 = 1000.0; //No, it's not 1024 - because we will calculate MB, not MiB. ;)
		const UNIT: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
		let mut current_multiplier = 1.0;
		let mut humanreadable_size = String::new();
		let size = self as f64;
		while (size) >= DIVISOR.powf(current_multiplier) {
			current_multiplier += 1.0;
		}
		humanreadable_size
			.push_str(&(format!("{:.2}", (size) / DIVISOR.powf(current_multiplier - 1.0))).to_string());
		humanreadable_size.push_str(&(UNIT[(current_multiplier - 1.0) as usize].to_string()));
		humanreadable_size
	}
}

impl HumanReadable for i64 {
	fn bytes_as_hrb(self) -> String {
		const DIVISOR: f64 = 1000.0; //No, it's not 1024 - because we will calculate MB, not MiB. ;)
		const UNIT: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
		let mut current_multiplier = 1.0;
		let mut humanreadable_size = String::new();
		let size = self as f64;
		while (size) >= DIVISOR.powf(current_multiplier) {
			current_multiplier += 1.0;
		}
		humanreadable_size
			.push_str(&(format!("{:.2}", (size) / DIVISOR.powf(current_multiplier - 1.0))).to_string());
		humanreadable_size.push_str(&(UNIT[(current_multiplier - 1.0) as usize].to_string()));
		humanreadable_size
	}
}

impl HumanReadable for i128 {
	fn bytes_as_hrb(self) -> String {
		const DIVISOR: f64 = 1000.0; //No, it's not 1024 - because we will calculate MB, not MiB. ;)
		const UNIT: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
		let mut current_multiplier = 1.0;
		let mut humanreadable_size = String::new();
		let size = self as f64;
		while (size) >= DIVISOR.powf(current_multiplier) {
			current_multiplier += 1.0;
		}
		humanreadable_size
			.push_str(&(format!("{:.2}", (size) / DIVISOR.powf(current_multiplier - 1.0))).to_string());
		humanreadable_size.push_str(&(UNIT[(current_multiplier - 1.0) as usize].to_string()));
		humanreadable_size
	}
}

impl HumanReadable for isize {
	fn bytes_as_hrb(self) -> String {
		const DIVISOR: f64 = 1000.0; //No, it's not 1024 - because we will calculate MB, not MiB. ;)
		const UNIT: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
		let mut current_multiplier = 1.0;
		let mut humanreadable_size = String::new();
		let size = self as f64;
		while (size) >= DIVISOR.powf(current_multiplier) {
			current_multiplier += 1.0;
		}
		humanreadable_size
			.push_str(&(format!("{:.2}", (size) / DIVISOR.powf(current_multiplier - 1.0))).to_string());
		humanreadable_size.push_str(&(UNIT[(current_multiplier - 1.0) as usize].to_string()));
		humanreadable_size
	}
}

impl HumanReadable for u8 {
	fn bytes_as_hrb(self) -> String {
		const DIVISOR: f32 = 1000.0; //No, it's not 1024 - because we will calculate MB, not MiB. ;)
		const UNIT: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
		let mut current_multiplier = 1.0;
		let mut humanreadable_size = String::new();
		let size = self as f32;
		while (size) >= DIVISOR.powf(current_multiplier) {
			current_multiplier += 1.0;
		}
		humanreadable_size
			.push_str(&(format!("{:.2}", (size) / DIVISOR.powf(current_multiplier - 1.0))).to_string());
		humanreadable_size.push_str(&(UNIT[(current_multiplier - 1.0) as usize].to_string()));
		humanreadable_size
	}
}

impl HumanReadable for u16 {
	fn bytes_as_hrb(self) -> String {
		const DIVISOR: f32 = 1000.0; //No, it's not 1024 - because we will calculate MB, not MiB. ;)
		const UNIT: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
		let mut current_multiplier = 1.0;
		let mut humanreadable_size = String::new();
		let size = self as f32;
		while (size) >= DIVISOR.powf(current_multiplier) {
			current_multiplier += 1.0;
		}
		humanreadable_size
			.push_str(&(format!("{:.2}", (size) / DIVISOR.powf(current_multiplier - 1.0))).to_string());
		humanreadable_size.push_str(&(UNIT[(current_multiplier - 1.0) as usize].to_string()));
		humanreadable_size
	}
}

impl HumanReadable for u32 {
	fn bytes_as_hrb(self) -> String {
		const DIVISOR: f64 = 1000.0; //No, it's not 1024 - because we will calculate MB, not MiB. ;)
		const UNIT: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
		let mut current_multiplier = 1.0;
		let mut humanreadable_size = String::new();
		let size = self as f64;
		while (size) >= DIVISOR.powf(current_multiplier) {
			current_multiplier += 1.0;
		}
		humanreadable_size
			.push_str(&(format!("{:.2}", (size) / DIVISOR.powf(current_multiplier - 1.0))).to_string());
		humanreadable_size.push_str(&(UNIT[(current_multiplier - 1.0) as usize].to_string()));
		humanreadable_size
	}
}

impl HumanReadable for u64 {
	fn bytes_as_hrb(self) -> String {
		const DIVISOR: f64 = 1000.0; //No, it's not 1024 - because we will calculate MB, not MiB. ;)
		const UNIT: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
		let mut current_multiplier = 1.0;
		let mut humanreadable_size = String::new();
		let size = self as f64;
		while (size) >= DIVISOR.powf(current_multiplier) {
			current_multiplier += 1.0;
		}
		humanreadable_size
			.push_str(&(format!("{:.2}", (size) / DIVISOR.powf(current_multiplier - 1.0))).to_string());
		humanreadable_size.push_str(&(UNIT[(current_multiplier - 1.0) as usize].to_string()));
		humanreadable_size
	}
}

impl HumanReadable for u128 {
	fn bytes_as_hrb(self) -> String {
		const DIVISOR: f64 = 1000.0; //No, it's not 1024 - because we will calculate MB, not MiB. ;)
		const UNIT: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
		let mut current_multiplier = 1.0;
		let mut humanreadable_size = String::new();
		let size = self as f64;
		while (size) >= DIVISOR.powf(current_multiplier) {
			current_multiplier += 1.0;
		}
		humanreadable_size
			.push_str(&(format!("{:.2}", (size) / DIVISOR.powf(current_multiplier - 1.0))).to_string());
		humanreadable_size.push_str(&(UNIT[(current_multiplier - 1.0) as usize].to_string()));
		humanreadable_size
	}
}

impl HumanReadable for usize {
	fn bytes_as_hrb(self) -> String {
		const DIVISOR: f64 = 1000.0; //No, it's not 1024 - because we will calculate MB, not MiB. ;)
		const UNIT: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
		let mut current_multiplier = 1.0;
		let mut humanreadable_size = String::new();
		let size = self as f64;
		while (size) >= DIVISOR.powf(current_multiplier) {
			current_multiplier += 1.0;
		}
		humanreadable_size
			.push_str(&(format!("{:.2}", (size) / DIVISOR.powf(current_multiplier - 1.0))).to_string());
		humanreadable_size.push_str(&(UNIT[(current_multiplier - 1.0) as usize].to_string()));
		humanreadable_size
	}
}