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
use std::io::prelude::*;
use std::fs::File;
use std::sync::{Arc,Mutex};

pub use functions::TS3Functions;
pub use errors::Error;
pub use callbacks::*;
pub use interface::*;
pub use definitions::*;
pub use state::*;

#[macro_export]
macro_rules! teamspeak3_plugin {
	($t:ty) => (

		fn register_plugin() {
			let has_plugin = { 
				let inni = ::ts3plugin::singleton();
				let guard = inni.plugin.lock().unwrap();
				(*guard).is_some()
			};
			if !has_plugin {
				let instance = <$t>::create_instance();
				let inni = ::ts3plugin::singleton();
				let mut data = inni.plugin.lock().unwrap();
				*data = Some(instance);
			}
		}

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
			register_plugin();

			let inni = ::ts3plugin::singleton();
			let mut guard = inni.plugin.lock().unwrap();
			if let Some(ref mut plugin) = *guard {
				plugin.register_client_functions(fs).ok();
			}
		}

		#[no_mangle]
		#[allow(non_snake_case)]
		pub fn ts3plugin_init() -> libc::c_int {
			let inni = ::ts3plugin::singleton();
			let mut guard = inni.plugin.lock().unwrap();
			if let Some(ref mut plugin) = *guard {
				match plugin.init() {
					Ok(_)		=>	0,
					Err(_)		=>	1,
				}
			} else {
				1
			}
		}

		#[no_mangle]
		#[allow(non_snake_case)]
		pub fn ts3plugin_shutdown() {
			// free stuff here?
			let singleton = ::ts3plugin::singleton();
			let mut guard = (*singleton.plugin).lock().unwrap();
			if let Some(ref mut plugin) = *guard {
				plugin.shutdown().ok();
			}
		}
	);
}
