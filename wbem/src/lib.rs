/*!
*/

extern crate winapi;
#[macro_use]
extern crate com_core as com;

extern crate com_types;
pub use com_types::{bstr, variant};

extern crate wbem_sys;

mod locator;
pub use locator::{ILocatorPtr, Locator};
mod services;
pub use services::{IServicesPtr};
mod enum_class_object;
pub use enum_class_object::{IEnumClassObjectPtr};
mod class_object;
pub use class_object::{IClassObjectPtr, ClassObjectEnumerator};
