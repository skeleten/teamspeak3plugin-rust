use std::ffi::CStr;

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
}
