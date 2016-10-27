/*!
*/

use ::std::{ptr, mem};

use ::wbem_sys::cli as wbem_cli;
use ::wbem_sys::hr as wbem_hr;

use ::com::hr::{HResult};
use ::com::{unknown, ComResult};
use ::bstr::{AsRawBStr, BString};

use ::variant::Variant;

com_ptr! {
	#[derive(Debug, Clone)]
	pub struct IClassObjectPtr(wbem_cli::IWbemClassObject): unknown::IUnknownPtr;
}
impl IClassObjectPtr {
	pub fn get<'s, N: AsRawBStr>(&'s self, name: N) -> ComResult<Option<Variant<'s>>> {
		unsafe {
			let mut var = mem::uninitialized();
			let hr = com_call!(Get(self, name.as_raw(), 0, &mut var, ptr::null_mut(), ptr::null_mut()));
			if hr == wbem_hr::WBEM_E_NOT_FOUND {
				Ok(None)
			}
			else {
				HResult::result_of(hr, || Some(Variant::from_raw(var)))
			}
		}
	}
	pub fn enumerate(&mut self, enum_flags: i32) -> ComResult<ClassObjectEnumerator> {
		unsafe {
			let hr = com_call!(BeginEnumeration(self, enum_flags));
			HResult::result_of(hr, move || ClassObjectEnumerator(self))
		}
	}
}

pub struct ClassObjectEnumerator<'a>(&'a mut IClassObjectPtr);
impl<'a> Drop for ClassObjectEnumerator<'a> {
	fn drop(&mut self) {
		unsafe {
			let _ = com_call!(EndEnumeration(self.0));
		}
	}
}
impl<'a> Iterator for ClassObjectEnumerator<'a> {
	type Item = ComResult<(BString, Variant<'a>)>;
	fn next(&mut self) -> Option<ComResult<(BString, Variant<'a>)>> {
		unsafe {
			let mut name = ptr::null_mut();
			let mut var = mem::uninitialized();
			let hr = com_call!(Next(self.0, 0, &mut name, &mut var, ptr::null_mut(), ptr::null_mut()));
			if hr == ::wbem_sys::hr::WBEM_S_NO_MORE_DATA {
				None
			}
			else {
				Some(HResult::result_of(hr, || (BString::from_raw(name), Variant::from_raw(var))))
			}
		}
	}
}
