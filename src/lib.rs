// src/lib.rs
#![crate_type = "dylib"]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

extern crate libc;
mod defsenum;
mod defsother;
mod functions;

use std::ffi::CString;
// use libc;
use defsenum::*;
use defsother::*;

use std::io::prelude::*;
use std::fs::File;

const PLUGIN_API_VERSION: libc::c_int			= 20;

static mut funcs: Option<functions::TS3Functions>		= None;

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
pub unsafe fn ts3plugin_setFunctionPointers(fs: functions::TS3Functions) {
	funcs = Some(fs);
}

#[no_mangle]
pub fn ts3plugin_init() -> libc::c_int {
	let mut f = File::create("C:\\tmp\\test.txt").ok().unwrap();
	f.write_all(b"initialised!").ok();

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

