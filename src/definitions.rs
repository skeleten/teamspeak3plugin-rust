extern crate libc;

// plugin_definitions.h

use libc::types::common::c99::int8_t as c_char;
use libc::types::common::c99::uint8_t as c_uchar;
use libc::types::common::c99::int32_t as c_int;
use libc::types::common::c99::uint32_t as c_uint;

const PLUGIN_MENU_BUFSZ: usize = 128;

enum PluginConfigureOffer {
	PLUGIN_OFFERS_NO_CONFIGURE,
	PLUGIN_OFFERS_CONFIGURE_NEW_THREAD,
	PLUGIN_OOFERS_CONFIGURE_QT_THREAD,
}

enum PluginMessageTarget {
	PLUGIN_MESSAGE_TARGET_SERVER,
	PLUGIN_MESSAGE_TARGET_CHANNEL,
}

enum PluginItemType {
	PLUGIN_SERVER,
	PLUGIN_CHANNEL,
	PLUGIN_CLIENT,
}

enum PluginMenuType {
	PLUGIN_MENU_TYPE_GLOBAL,
	PLUGIN_MENU_TYPE_CHANNEL,
	PLUGIN_MENU_TYPE_CLIENT,
}

enum PluginGuiProfile {
	PLUGIN_GUI_SOUND_CAPTURE,
	PLUGIN_GUI_SOUND_PLAYBACK,
	PLUGIN_GUI_HOTKEY,
	PLUGIN_GUI_SOUNDPACK,
	PLUGIN_GUI_IDENTITY,
}

enum PluginConnectTab {
	PLUGIN_CONNECT_TAB_NEW,
	PLUGIN_CONNECT_TAB_CURRENT,
	PLUGIN_CONNECT_TAB_NEW_IF_CURRENT_CONNECTED,
}

struct PluginMenuItem {
	Type:			PluginMenuType,
	id:				c_uint,
	text:			[c_char; PLUGIN_MENU_BUFSZ],
	icon:			[c_char; PLUGIN_MENU_BUFSZ],
}

struct PluginBookmarkItem {
	name:			[c_char],
	isFolder:		c_uchar,
	reserved:		[c_uchar; 3],
	// union this ?!?
	name:			[c_char],
	folder:			*mut PluginBookmarkList,
}

struct PluginBookmarkList {
	itemcount:		c_int,
	items:			[PluginBookmarkItem; 1],
}
