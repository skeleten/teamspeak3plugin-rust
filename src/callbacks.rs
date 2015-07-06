use std::ffi::*;
use libc::*;
use state::*;
use definitions::*;
use interface::*;

// TODO: create callbacks here and wrap them

#[no_mangle]
pub unsafe fn ts3plugin_onConnectStatusChangeEvent(server_handler_id: c_ulong, newStatus: c_int, errorNumber: c_uint) {
	// TODO

	let mut singleton = ::state::singleton();
	let mut guard = singleton.plugin.lock().unwrap();

	if let Some(ref mut p) = *guard {
		p.on_connect_status_change_event(
			ServerConnectionHandler(server_handler_id as u64),
			newStatus as i32,
			errorNumber as u32);
	}
}

#[no_mangle]
pub unsafe fn ts3plugin_onNewChannelEvent(server_handler_id: c_ulong, channel_id: c_ulong, channel_parent_id: c_ulong) {
	// TODO
}

#[no_mangle]
pub unsafe fn ts3plugin_onNewChannelCreatedEvent(server_handler_id: c_ulong, channel_id: c_ulong, channel_parent_id: c_ulong, invoker_id: c_ushort, invoker_name: *const c_char, invoker_uniq_ident: *const c_char) {
	// TODO
}

#[no_mangle]
pub unsafe fn ts3plugin_onDelChannelEvent(server_handler_id: c_ulong, channel_id: c_ulong, invoker_id: c_ushort, invoker_name: *const c_char, invoker_uniq_ident: *const c_char) {
	// TODO
}

#[no_mangle]
pub unsafe fn ts3plugin_onChannelMoveEvent(server_handler_id: c_ulong, channel_id: c_ulong, new_parent_id: c_ulong, invoker_id: c_ushort, invoker_name: *const c_char, invoker_uniq_ident: *const c_char) {
	// TODO
}
