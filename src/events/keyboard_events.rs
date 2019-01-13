use super::Event;
use super::EventBox;
use super::EventListener;
use crate::input::Button;

pub struct KeyPressedEvent {
	handled: bool,
	button: Button,
	repeat: bool,
}

impl KeyPressedEvent {
	pub fn new(button: Button, repeat: bool) -> EventBox {
		Box::new(Self {
			handled: false,
			button,
			repeat,
		})
	}
}

impl Event for KeyPressedEvent {
	fn is_handled(&self) -> bool {
		self.handled
	}
	fn dispatch(&mut self, listener: &mut EventListener) {
		debug_assert!(!self.handled);
		self.handled = listener.on_key_press(self.button, self.repeat);
	}
}

pub struct KeyReleasedEvent {
	handled: bool,
	button: Button,
}

impl KeyReleasedEvent {
	pub fn new(button: Button) -> EventBox {
		Box::new(Self {
			handled: false,
			button,
		})
	}
}

impl Event for KeyReleasedEvent {
	fn is_handled(&self) -> bool {
		self.handled
	}
	fn dispatch(&mut self, listener: &mut EventListener) {
		debug_assert!(!self.handled);
		self.handled = listener.on_key_release(self.button);
	}
}

pub struct CharWrittenEvent {
	handled: bool,
	which: char,
}

impl CharWrittenEvent {
	pub fn new(which: char) -> EventBox {
		Box::new(Self {
			handled: false,
			which,
		})
	}
}

impl Event for CharWrittenEvent {
	fn is_handled(&self) -> bool {
		self.handled
	}
	fn dispatch(&mut self, listener: &mut EventListener) {
		debug_assert!(!self.handled);
		self.handled = listener.on_char_written(self.which);
	}
}
