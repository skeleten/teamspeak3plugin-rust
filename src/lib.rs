// src/lib.rs
// #![crate_type = "dylib"]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

extern crate libc;

mod errors;
mod definitions;
mod functions;
mod interface;
mod callbacks;
mod state;

use std::ffi::CString;
// use libc;

use std::io::prelude::*;
use std::fs::File;
use std::sync::{Arc,Mutex};

pub use functions::TS3Functions;
pub use interface::Plugin;
// pub use callbacks::*; // empty atm.
pub use definitions::*;

#[macro_export]
macro_rules! teamspeak3_plugin {
	($t:ty) => (
		pub fn ts3plugin_name() -> *const libc::c_char {
			use std::ffi::CString;
			static cstr: CString = CString::new(<$t>::name()).unwrap();
			cstr.as_ptr()
		}

		#[no_mangle]
		pub fn ts3plugin_version() -> *const libc::c_char {
			use std::ffi::CString;
			static cstr: CString = CString::new(<$t>::version()).unwrap();
			cstr.as_ptr()
		}

		#[no_mangle]
		pub fn ts3plugin_apiVersion() -> libc::c_int {
			<$t>::api_version() as ::libc::c_int
		}

		#[no_mangle]
		pub fn ts3plugin_author() -> *const libc::c_char {
			use std::ffi::CString;
			static cstr: CString = CString(<$t>::author()).unwrap();
			cstr.as_ptr()
		}

		#[no_mangle]
		pub fn ts3plugin_description() -> *const libc::c_char {
			use std::ffi::CString;
			static cstr: CString = CString::new(<$t>::description()).unwrap();
			cstr.as_ptr()
		}

		#[no_mangle]
		pub unsafe fn ts3plugin_setFunctionPointers(fs: ::ts3plugin::TS3Functions) {
			use std::sync::{Arc,Mutex};
			::ts3plugin::state::singleton().functions = Arc::new(Mutex::new(Some(fs)));
		}

		#[no_mangle]
		pub fn ts3plugin_init() -> libc::c_int {
			use std::sync::{Arc,Mutex};
			use ::ts3plugin::state::singleton;
			singleton().plugin = Arc::new(Mutex::new(Box::new(<$t>::create_isntance())));

			0
		}

		#[no_mangle]
		pub fn ts3plugin_shutdown() {
			// free stuff here?
		}
	);
}
