/*!
*/

use ::std::ptr;

use ::bstr::{AsRawBStr};
use ::com::hr::{HResult};
use ::com::{unknown, ComResult, ComPtr};

use ::wbem_sys::cli as wbem_cli;

com_ptr! {
	#[derive(Clone, Debug)]
	pub struct IServicesPtr(wbem_cli::IWbemServices): unknown::IUnknownPtr;
}
impl IServicesPtr {
	pub fn exec_query<L: AsRawBStr, Q: AsRawBStr>(&self, language: L, query: Q, flags: i32) -> ComResult<::IEnumClassObjectPtr> {
		unsafe {
			let mut p_enum = ptr::null_mut();
			let hr = com_call!(ExecQuery(
				self,
				language.as_raw(),
				query.as_raw(),
				flags,
				ptr::null_mut(),
				&mut p_enum
			));
			HResult::result_of(hr, || ::IEnumClassObjectPtr::from_ptr(p_enum))
		}
	}
}
