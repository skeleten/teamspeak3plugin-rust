use std::ffi::CStr;
use ::definitions::*;
use libc::*;
use std::ffi::CString;

/// Invoker of a request
pub struct Invoker {
	id:				u16,
	name:			String,
	unique_id:		String,
}

impl Invoker {
	/// Creates a new Invoker structure by converting the libc-types
	pub fn new(id: c_ushort, name: *const c_char, unique_id: *const c_char) -> Invoker {
		let m_id = id as u16;
		let name_cstr = unsafe { CStr::from_ptr(name) };
		let name_str = String::from_utf8_lossy(name_cstr.to_bytes()).into_owned();
		let unique_id_cstr = unsafe { CStr::from_ptr(unique_id) };
		let unique_id_str = String::from_utf8_lossy(unique_id_cstr.to_bytes()).into_owned();
		Invoker {
			id:			m_id,
			name:		name_str,
			unique_id:	unique_id_str,
		}
	}
}

/// Trait that has to be implemented by a plugin.
/// It determines the display values, just as Name, Author, etc.
pub trait PluginDescription {
	const NAME: &'static str;
	const VERSION: &'static str;
	const AUTHOR: &'static str;
	const DESCRIPTION: &'static str;
	const API_VERSION: i32 = 20;

	fn create_instance() -> Box<Plugin>;
}

// we allow unused variables for the function-prototypes here
#[allow(unused_variables)]
/// Core trait of any plugin.
/// the functions ```init``` as well as ```shutdown``` have to be implemented
/// the rest serve as callbacks and have a default empty implementation
pub trait Plugin: ::std::marker::Sync {
	fn init(&mut self) -> Result<(), ()>;
	fn shutdown(&mut self) -> Result<(), ()>;
	
	fn register_client_functions(&mut self, funcs: ::TS3Functions) -> Result<(), ()>;

	fn on_connect_status_change_event(&mut self, handler: ServerConnectionHandler, new_status: i32, error_number: u32) {
	}

	fn on_new_channel_event(&mut self, handler: ServerConnectionHandler, channel_id: u64, channel_parent_id: u64) {
	}

	fn on_new_channel_created_event(&mut self, handler: ServerConnectionHandler, channel_id: u64, channel_parent_id: u64, invoker: Invoker) {
	}

	fn on_del_channel_event(&mut self, handler: ServerConnectionHandler, channel_id: u64, invoker: Invoker)  {
	}

	fn on_channel_move_event(&mut self, handler: ServerConnectionHandler, channel_id: u64, new_parent_id: u64, invoker: Invoker) {
	}

	// few missing here

	fn on_client_move_event(&mut self, handler: ServerConnectionHandler, client_id: u16, old_channel_id: u64, new_channel_id: u64, visibility: i32, move_message: String) {
	}
}