use std::ffi::CStr;

pub trait Plugin: ::std::marker::Sync {
	fn create_instance() -> Self;
	fn name() -> String;
	fn version() -> String;
	fn api_version() -> i32;
	fn author() -> String;
	fn description() -> String;

	fn init(&self) -> Result<(), ()>;
}
