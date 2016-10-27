extern crate com_core as com;
extern crate com_types;
extern crate wbem;

use com::com::*;
use com_types::bstr::*;

#[test]
fn connect() {
	// Step 1: Initialize COM
	let com = Com::initialize_ex(COINIT_MULTITHREADED).unwrap();
	// Step 2: Set general COM security levels
	let com = com.initialize_security().unwrap();
	// Step 3: Create the locator
	let loc = com.create_instance::<wbem::ILocatorPtr, wbem::Locator>().unwrap();
	// Step 4: Connect to WMI
	let namespace = BArray::<[_; 16]>::from("ROOT\\CimV2");
	let svc = loc.connect_server(namespace, NullBStr, NullBStr, NullBStr, 0, NullBStr).unwrap();
	// Step 5: Set security levels on the proxy
	com.set_proxy_blanket(&svc).unwrap();

	let qenum = svc.exec_query(
		BArray::<[_; 6]>::from("WQL"),
		BString::from("SELECT * FROM Win32_BaseBoard"),
		0x20|0x10
	).unwrap();

	while let Ok(Some(mut class_object)) = qenum.next(com::timeout::INFINITE) {
		if let Ok(Some(config_options)) = class_object.get(BArray::<[_; 16]>::from("ConfigOptions")) {
			println!("tag: {:?}", config_options.tag());
			// unsafe {
			// 	let p_safe_array = ::std::mem::transmute_copy(&(*config_options.as_ptr()).data1);
			// 	let safe_array = com_types::safe_array::SafeArr::from_raw(p_safe_array);
			// 	println!("{:#?}", safe_array);
			// }
			// println!("{:?}", config_options);
		}
		for it in class_object.enumerate(64).unwrap() {
			print!("{:?},\n", it);
		}
		println!("");
	}
}
