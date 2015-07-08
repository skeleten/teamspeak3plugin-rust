// plugin_definitions.h
use libc::*;

pub const PLUGIN_MENU_BUFSZ: usize = 128;

pub enum PluginConfigureOffer {
	PLUGIN_OFFERS_NO_CONFIGURE,
	PLUGIN_OFFERS_CONFIGURE_NEW_THREAD,
	PLUGIN_OFFERS_CONFIGURE_QT_THREAD,
}

pub enum PluginMessageTarget {
	PLUGIN_MESSAGE_TARGET_SERVER,
	PLUGIN_MESSAGE_TARGET_CHANNEL,
}

pub enum PluginItemType {
	PLUGIN_SERVER,
	PLUGIN_CHANNEL,
	PLUGIN_CLIENT,
}

pub enum PluginMenuType {
	PLUGIN_MENU_TYPE_GLOBAL,
	PLUGIN_MENU_TYPE_CHANNEL,
	PLUGIN_MENU_TYPE_CLIENT,
}

pub enum PluginGuiProfile {
	PLUGIN_GUI_SOUND_CAPTURE,
	PLUGIN_GUI_SOUND_PLAYBACK,
	PLUGIN_GUI_HOTKEY,
	PLUGIN_GUI_SOUNDPACK,
	PLUGIN_GUI_IDENTITY,
}

pub enum PluginConnectTab {
	PLUGIN_CONNECT_TAB_NEW,
	PLUGIN_CONNECT_TAB_CURRENT,
	PLUGIN_CONNECT_TAB_NEW_IF_CURRENT_CONNECTED,
}

pub struct PluginMenuItem {
	Type:			PluginMenuType,
	id:				c_uint,
	text:			[c_char; PLUGIN_MENU_BUFSZ],
	icon:			[c_char; PLUGIN_MENU_BUFSZ],
}

pub struct PluginBookmarkItem {
	name:			*mut c_char,// [c_char],
	isFolder:		c_uchar,
	reserved:		[c_uchar; 3],
	// union this ?!?
	// TODO: create struct w/ the fixed size (no idea what size it really is, but oh well)
	// name_2:			[c_char; 1],
	folder:			*mut PluginBookmarkList,
}

pub struct PluginBookmarkList {
	itemcount:		c_int,
	items:			[PluginBookmarkItem; 1],
}


// public_definitions.h

pub const TS3_MAX_SIZE_CHANNEL_NAME: usize					= 40;
pub const TS3_MAX_SIZE_VIRTUALSERVER_NAME: usize			= 64;
pub const TS3_MAX_SIZE_CLIENT_NICKNAME: usize				= 64;
pub const TS3_MIN_SIZE_CLIENT_NICKNAME: usize				= 3;
pub const TS3_MAX_SIZE_REASON_MESSAGE: usize				= 80;
pub const TS3_MAX_SIZE_TEXTMESSAGE: usize					= 1024;


// TODO

/// Logging level, for use with the function ```functions::TS3Functions::logMessage```
pub enum LogLevel {
	/// These messages stop the program
	LogLevel_CRITICAL = 0,
	/// Everything that is really bad, but not so bad we need to shut down
	LogLevel_ERROR,
	/// Everything that *might* be bad
	LogLevel_WARNING,
	/// Output that might help find a problem
	LogLevel_DEBUG,
	/// Informational output, like "starting database version x.y.z"
	LogLevel_INFO,
	/// Developer only output (will not be displayed in release mode)
	LogLevel_DEVEL
}


// own stuff


// newtypes
pub struct ServerConnectionHandler(pub u64);


impl Into<c_ulong> for ServerConnectionHandler {
	fn into(self) -> c_ulong {
		let ServerConnectionHandler(h) = self;
		h as c_ulong
	}
}

impl From<c_ulong> for ServerConnectionHandler {
	fn from(data: c_ulong) -> Self {
		ServerConnectionHandler(data as u64)
	}
}


pub struct UserHandler(pub u16);

impl Into<c_ushort> for UserHandler {
	fn into(self) -> c_ushort {
		let UserHandler(id) = self;
		id as c_ushort
	}
}