use crate::events::EventListener;

pub trait Layer: EventListener {
	fn on_attach(&mut self) {}
	fn on_detach(&mut self) {}
	fn on_update(&mut self) {}
	// fn on_event(&mut self, _event: &Event) {}

	#[cfg(debug_assertions)]
	fn get_name(&self) -> &str;
}
