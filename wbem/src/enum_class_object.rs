/*!
*/

use ::std::ptr;

use ::wbem_sys::cli as wbem_cli;
use ::com::hr::{HResult};
use ::com::{unknown, ComResult, ComPtr};

use ::IClassObjectPtr;
use ::com::timeout::Timeout;

//----------------------------------------------------------------

com_ptr! {
	#[derive(Clone, Debug)]
	pub struct IEnumClassObjectPtr(wbem_cli::IEnumWbemClassObject): unknown::IUnknownPtr;
}
impl IEnumClassObjectPtr {
	pub fn reset(&self) -> ComResult<()> {
		unsafe {
			let hr = com_call!(Reset(self));
			HResult::result(hr, ())
		}
	}
	pub fn next(&self, timeout: Timeout) -> ComResult<Option<IClassObjectPtr>> {
		unsafe {
			let mut class_object = ptr::null_mut();
			let mut returned = 0;
			let hr = com_call!(Next(self, timeout.into(), 1, &mut class_object, &mut returned));
			if returned == 1 {
				Ok(Some(IClassObjectPtr::from_ptr(class_object)))
			}
			else if hr == ::wbem_sys::hr::WBEM_S_FALSE {
				Ok(None)
			}
			else {
				Err(hr.into())
			}
		}
	}
	pub fn dup(&self) -> ComResult<IEnumClassObjectPtr> {
		unsafe {
			let mut p_enum_class_object = ptr::null_mut();
			let hr = com_call!(Clone(self, &mut p_enum_class_object));
			HResult::result_of(hr, || IEnumClassObjectPtr::from_ptr(p_enum_class_object))
		}
	}
	pub fn skip(&self, timeout: Timeout, count: u32) -> ComResult<()> {
		unsafe {
			let hr = com_call!(Skip(self, timeout.into(), count));
			HResult::result(hr, ())
		}
	}
}
