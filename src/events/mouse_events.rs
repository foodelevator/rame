use super::Event;
use super::EventBox;
use super::EventListener;
use crate::input::Button;
use crate::vecs::Vec2;

pub struct MousePressedEvent {
	handled: bool,
	button: Button,
}

impl MousePressedEvent {
	pub fn new(button: Button) -> EventBox {
		Box::new(Self {
			handled: false,
			button,
		})
	}
}

impl Event for MousePressedEvent {
	fn is_handled(&self) -> bool {
		self.handled
	}
	fn dispatch(&mut self, listener: &mut EventListener) {
		debug_assert!(!self.handled);
		self.handled = listener.on_mouse_press(self.button);
	}
}

pub struct MouseReleasedEvent {
	handled: bool,
	button: Button,
}

impl MouseReleasedEvent {
	pub fn new(button: Button) -> EventBox {
		Box::new(Self {
			handled: false,
			button,
		})
	}
}

impl Event for MouseReleasedEvent {
	fn is_handled(&self) -> bool {
		self.handled
	}
	fn dispatch(&mut self, listener: &mut EventListener) {
		debug_assert!(!self.handled);
		self.handled = listener.on_mouse_release(self.button);
	}
}

pub struct MouseMovedEvent {
	handled: bool,
	position: Vec2,
}

impl MouseMovedEvent {
	pub fn new(position: Vec2) -> EventBox {
		Box::new(Self {
			handled: false,
			position,
		})
	}
}

impl Event for MouseMovedEvent {
	fn is_handled(&self) -> bool {
		self.handled
	}
	fn dispatch(&mut self, listener: &mut EventListener) {
		debug_assert!(!self.handled);
		self.handled = listener.on_mouse_move(self.position);
	}
}

pub struct MouseScrolledEvent {
	handled: bool,
	delta: Vec2,
}

impl MouseScrolledEvent {
	pub fn new(delta: Vec2) -> EventBox {
		Box::new(Self {
			handled: false,
			delta,
		})
	}
}

impl Event for MouseScrolledEvent {
	fn is_handled(&self) -> bool {
		self.handled
	}
	fn dispatch(&mut self, listener: &mut EventListener) {
		debug_assert!(!self.handled);
		self.handled = listener.on_mouse_scroll(self.delta);
	}
}
