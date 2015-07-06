use std::ffi::CStr;
use ::definitions::*;

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

	fn on_new_channel_created_event(&mut self, handler: ServerConnectionHandler, channel_id: u64, invoker_id: u16, invoker_name: String, invoker_unique_id: String) {
	}

	fn on_del_channel_event(&mut self, handler: ServerConnectionHandler, channel_id: u64, invoker_id: u16, invoker_name: String, invoker_unique_id: String)  {
	}

	fn on_channel_move_event(&mut self, handler: ServerConnectionHandler, channel_id: u64, new_parent_id: u64, invoker_id: u16, invoker_name: String, invoker_unique_id: String) {
	}
}