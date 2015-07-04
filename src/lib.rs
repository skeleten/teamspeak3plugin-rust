// src/lib.rs
// #![crate_type = "dylib"]
#![feature(associated_consts)]
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
pub use interface::*;
// pub use callbacks::*; // empty atm.
pub use definitions::*;
pub use state::*;

#[macro_export]
macro_rules! teamspeak3_plugin {
	($t:ty) => (
		#[no_mangle]
		#[allow(non_snake_case)]
		pub fn ts3plugin_name() -> *const libc::c_char {
			static CSTR: &'static str = <$t>::NAME;
			CSTR.as_ptr() as *const ::libc::c_char
		}

		#[no_mangle]
		#[allow(non_snake_case)]
		pub fn ts3plugin_version() -> *const libc::c_char {
			static CSTR: &'static str = <$t>::VERSION;
			CSTR.as_ptr() as *const ::libc::c_char
		}

		#[no_mangle]
		#[allow(non_snake_case)]
		pub fn ts3plugin_author() -> *const libc::c_char {
			static CSTR: &'static str = <$t>::AUTHOR;
			CSTR.as_ptr() as *const ::libc::c_char
		}

		#[no_mangle]
		#[allow(non_snake_case)]
		pub fn ts3plugin_description() -> *const libc::c_char {
			static CSTR: &'static str = <$t>::DESCRIPTION;
			CSTR.as_ptr() as *const ::libc::c_char
		}

		#[no_mangle]
		#[allow(non_snake_case)]
		pub fn ts3plugin_apiVersion() -> libc::c_int {
			<$t>::API_VERSION as ::libc::c_int
		}

		#[no_mangle]
		#[allow(non_snake_case)]
		pub unsafe fn ts3plugin_setFunctionPointers(fs: ::ts3plugin::TS3Functions) {
			use std::sync::{Arc,Mutex};
			::ts3plugin::singleton().functions = Arc::new(Mutex::new(Some(fs)));
		}

		#[no_mangle]
		#[allow(non_snake_case)]
		pub fn ts3plugin_init() -> libc::c_int {
			use std::sync::{Arc,Mutex};
			let instance = <$t>::create_instance();
			instance.init();
			::ts3plugin::singleton().plugin = Arc::new(Mutex::new(Some(instance)));

			0
		}

		#[no_mangle]
		#[allow(non_snake_case)]
		pub fn ts3plugin_shutdown() {
			// free stuff here?
		}
	);
}
