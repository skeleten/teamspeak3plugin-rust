use std::ffi::CStr;
use ::definitions::*;
use libc::*;
use std::ffi::CString;

pub struct Invoker {
	id:				u16,
	name:			String,
	unique_id:		String,
}

impl Invoker {
	pub fn new(id: c_ushort, name: *const c_char, unique_id: *const c_char) -> Invoker {
		let m_id = id as u16;
		let name_str = CString::new(name).unwrap().to_string();
		let unique_id_str = CString::new(unique_id).unwrap().to_string();
		Invoker {
			id:			m_id,
			name:		name_str,
			unique_id:	unique_id_str,
		}
	}
}

pub trait PluginDescription {
	const NAME: &'static str;
	const VERSION: &'static str;
	const AUTHOR: &'static str;
	const DESCRIPTION: &'static str;
	const API_VERSION: i32 = 20;

	fn create_instance() -> Box<Plugin>;
}

pub trait Plugin: ::std::marker::Sync {
	fn init(&self) -> Result<(), ()>;
	fn register_client_functions(&mut self, funcs: ::TS3Functions) -> Result<(), ()>;

	fn on_connect_status_change_event(&mut self, handler: ServerConnectionHandler, new_status: i32, error_number: u32) {
	}

	fn on_new_channel_event(&mut self, handler: ServerConnectionHandler, channel_id: u64, channel_parent_id: u64) {
	}

	fn on_new_channel_created_event(&mut self, handler: ServerConnectionHandler, channel_id: u64, invoker: Invoker) {
	}

	fn on_del_channel_event(&mut self, handler: ServerConnectionHandler, channel_id: u64, invoker: Invoker)  {
	}

	fn on_channel_move_event(&mut self, handler: ServerConnectionHandler, channel_id: u64, new_parent_id: u64, invoker: Invoker) {
	}
}