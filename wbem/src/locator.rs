/*!
*/

use ::std::{ptr};

use ::bstr::{AsRawBStr};
use ::com::hr::{HResult};
use ::com::{unknown, ComResult, ComPtr};

use ::wbem_sys::cli as wbem_cli;

pub use ::wbem_sys::cli::WbemLocator as Locator;

com_ptr! {
	#[derive(Clone, Debug)]
	pub struct ILocatorPtr(wbem_cli::IWbemLocator): unknown::IUnknownPtr;
}
impl ILocatorPtr {
	pub fn connect_server<'a, N, U, P, L, A>(&self, network_resource: N, user: U, password: P, locale: L, security_flags: i32, authority: A) -> ComResult<::IServicesPtr>
		where N: AsRawBStr, U: AsRawBStr, P: AsRawBStr, L: AsRawBStr, A: AsRawBStr
	{
		unsafe {
			let mut p_services = ptr::null_mut();
			let hr = com_call!(ConnectServer(
				self,
				network_resource.as_raw(),
				user.as_raw(),
				password.as_raw(),
				locale.as_raw(),
				security_flags,
				authority.as_raw(),
				ptr::null_mut(),
				&mut p_services
			));
			HResult::result_of(hr, || ::IServicesPtr::from_ptr(p_services))
		}
	}
}
