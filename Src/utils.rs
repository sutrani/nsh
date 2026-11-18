// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ffi::{CStr, c_char};

unsafe extern "C" {
	pub fn dputs(message: *const c_char, ...);
}

#[macro_export]
macro_rules! DPUTS {
	($cond:expr, $msg:literal) => {
		if $cond {
			const MSG: &CStr = $msg;
			unsafe {
				dputs(MSG.as_ptr());
			}
		}
	};
}
