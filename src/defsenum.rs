// plugin_definitions.h
use libc::*;

pub const PLUGIN_MENU_BUFSZ: usize = 128;

pub enum PluginConfigureOffer {
	PLUGIN_OFFERS_NO_CONFIGURE,
	PLUGIN_OFFERS_CONFIGURE_NEW_THREAD,
	PLUGIN_OOFERS_CONFIGURE_QT_THREAD,
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
	name_2:			[c_char; 1],
	folder:			*mut PluginBookmarkList,
}

pub struct PluginBookmarkList {
	itemcount:		c_int,
	items:			[PluginBookmarkItem; 1],
}
