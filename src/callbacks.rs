use std::ffi::*;
use libc::*;
use state::*;
use definitions::*;
use ::interface::*;

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
	let singleton = ::state::singleton();
	let mut guard = singleton.plugin.lock().unwrap();

	if let Some(ref mut p) = *guard {
		let handler = ServerConnectionHandler(server_handler_id as u64);
		p.on_new_channel_event(
			handler,
			channel_id as u64,
			channel_parent_id as u64);
	}
}

#[no_mangle]
pub unsafe fn ts3plugin_onNewChannelCreatedEvent(
			server_handler_id: c_ulong, 
			channel_id: c_ulong, 
			channel_parent_id: c_ulong, 
			invoker_id: c_ushort, 
			invoker_name: *const c_char, 
			invoker_uniq_ident: *const c_char) {

	let mut singleton = ::state::singleton();
	let mut guard = (*singleton.plugin).lock().unwrap();
	if let Some(ref mut plugin) = *guard {
		let handler = ServerConnectionHandler::from(server_handler_id);
		let invoker = Invoker::new(invoker_id, invoker_name, invoker_uniq_ident);
		plugin.on_new_channel_created_event(
			handler,
			channel_id as u64,
			channel_parent_id as u64,
			invoker);
	}
}

#[no_mangle]
pub unsafe fn ts3plugin_onDelChannelEvent(
			server_handler_id: c_ulong, 
			channel_id: c_ulong, 
			invoker_id: c_ushort, 
			invoker_name: *const c_char, 
			invoker_uniq_ident: *const c_char) {

	let singleton = ::state::singleton();
	let mut guard = (*singleton.plugin).lock().unwrap();
	if let Some(ref mut plugin) = *guard {
		let handler = ServerConnectionHandler::from(server_handler_id);
		let invoker = Invoker::new(invoker_id, invoker_name, invoker_uniq_ident);
		plugin.on_del_channel_event(
			handler,
			channel_id as u64,
			invoker);
	}
}

#[no_mangle]
pub unsafe fn ts3plugin_onChannelMoveEvent(
			server_handler_id: c_ulong, 
			channel_id: c_ulong, 
			new_parent_id: c_ulong, 
			invoker_id: c_ushort, 
			invoker_name: *const c_char, 
			invoker_uniq_ident: *const c_char) {
	
	let singleton = ::state::singleton();
	let mut guard = (*singleton.plugin).lock().unwrap();
	if let Some(ref mut plugin) = *guard {
		let handler = ServerConnectionHandler::from(server_handler_id);
		let invoker = Invoker::new(invoker_id, invoker_name, invoker_uniq_ident);
		plugin.on_channel_move_event(
			handler,
			channel_id as  u64,
			new_parent_id as u64,
			invoker);
	}
}

// few more

#[no_mangle]
pub unsafe fn ts3plugin_onClientMoveEvent(
	handler: c_ulong,
	client_id: c_ushort,
	old_channel_id: c_ulong,
	new_channel_id: c_ulong,
	visibility: c_int,
	move_message: *const c_char) {

	let handler = ServerConnectionHandler::from(handler);
	let move_message_cstr = CStr::from_ptr(move_message);
	let move_message_str = String::from_utf8_lossy(move_message_cstr.to_bytes()).into_owned();

	let singleton = ::state::singleton();
	let mut guard = singleton.plugin.lock().unwrap();
	if let Some(ref mut plugin) = *guard {
		plugin.on_client_move_event(
			handler,
			client_id as u16,
			old_channel_id as u64,
			new_channel_id as u64,
			visibility as i32,
			move_message_str);
	}
}