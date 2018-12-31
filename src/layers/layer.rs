use crate::events::EventListener;

pub trait Layer {
	fn on_attach(&mut self) {}
	fn on_detach(&mut self) {}
	fn get_event_listener(&mut self) -> &mut EventListener;
}
