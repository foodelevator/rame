use super::{Event, EventBox, EventListener};

pub struct AppUpdateEvent;

impl AppUpdateEvent {
	pub fn new() -> EventBox {
		Box::new(AppUpdateEvent)
	}
}

impl Event for AppUpdateEvent {
	fn is_handled(&self) -> bool {
		false
	}
	fn dispatch(&mut self, listener: &mut EventListener) {
		listener.on_update();
	}
}

pub struct AppRenderEvent;

impl AppRenderEvent {
	pub fn new() -> EventBox {
		Box::new(AppRenderEvent)
	}
}

impl Event for AppRenderEvent {
	fn is_handled(&self) -> bool {
		false
	}
	fn dispatch(&mut self, listener: &mut EventListener) {
		listener.on_render();
	}
}
