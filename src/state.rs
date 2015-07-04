use functions::TS3Functions;
use interface::Plugin;

pub struct GlobalState {
	functions:			functions::TS3Functions,
	instance:			Plugin,
}

static mut __functions:		Option<TS3Functions>		= None;
static mut __instance:		Option<Plugin>				= None;

fn create_state() -> GlobalState {
	GlobalState {
		functions:		__functions.unwrap(),
		instance: 		__instance.unwrap(),
	}
}

lazy_init! {
	pub static ref Instance:		GlobalState 		= create_state();
}
