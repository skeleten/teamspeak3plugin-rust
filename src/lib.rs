// src/lib.rs
#![crate_type = "dylib"]
extern crate libc;

use std::ffi::CString;
// use libc;

const PLUGIN_API_VERSION: libc::c_int			= 20;

#[no_mangle]
pub fn ts3plugin_name() -> *const libc::c_char {
	CString::new("rust_plugin_test").unwrap().as_ptr()
}

#[no_mangle]
pub fn ts3plugin_version() -> *const libc::c_char {
	CString::new("0.1").unwrap().as_ptr()
}

#[no_mangle]
pub fn ts3plugin_apiVersion() -> libc::c_int {
	PLUGIN_API_VERSION
}

#[no_mangle]
pub fn ts3plugin_author() -> *const libc::c_char {
	CString::new("skeleten <skele@ymail.com>").unwrap().as_ptr()
}

#[no_mangle]
pub fn ts3plugin_description() -> *const libc::c_char {
	CString::new("Rust plugin test!").unwrap().as_ptr()
}

#[no_mangle]
pub fn ts3plugin_setFunctionPointers(/* ??? */) {
	// TODO!!!
}

#[no_mangle]
pub fn ts3plugin_init() -> libc::c_int {
	// should return 0 on success, 1 on failure

	/* getAppPath, getResourcesPath, getConfigPath, getPluginPath
	 * and stuff should be done here
	 */
	0
}

#[no_mangle]
pub fn ts3plugin_shutdown() {
	// free stuff here?
}

