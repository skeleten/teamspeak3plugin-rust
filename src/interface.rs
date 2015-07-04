use std;

pub trait Plugin: std::marker::Sync {
	fn create_instance() -> Plugin;

	fn init(&self) -> Result<(), ()>;
}
