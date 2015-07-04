trait Plugin {
	fn create_instance() -> Plugin;

	fn init(&self) -> Result<(), ()>;
}