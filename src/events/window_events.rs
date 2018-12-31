use super::Event;
use super::EventListener;
use super::EventBox;

// Window events should always propagate further, therefore
// their `is_handled` function always return `false`

pub struct WindowClosedEvent;

impl WindowClosedEvent {
	pub fn new() -> EventBox {
		Box::new(WindowClosedEvent)
	}
}

impl Event for WindowClosedEvent {
	fn is_handled(&self) -> bool {
		false
	}
	fn dispatch(&mut self, listener: &mut EventListener) {
		listener.on_window_closed();
	}
}

pub struct WindowResizedEvent {
	width: u32,
	height: u32,
}

impl WindowResizedEvent {
	pub fn new(width: u32, height: u32) -> EventBox {
		Box::new(Self { width, height })
	}
}

impl Event for WindowResizedEvent {
	fn is_handled(&self) -> bool {
		false
	}
	fn dispatch(&mut self, listener: &mut EventListener) {
		listener.on_window_resize(self.width, self.height);
	}
}
