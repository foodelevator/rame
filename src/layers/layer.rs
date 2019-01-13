use crate::events::EventListener;

pub trait Layer: EventListener {
	fn on_attach(&mut self) {}
	fn on_detach(&mut self) {}
}
