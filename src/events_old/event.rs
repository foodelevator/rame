use crate::input::button::Button;
use crate::vecs::Vec2;

#[derive(Debug, Clone)]
pub struct Event {
	handled: bool,
	e_type: EventType,
}

impl Event {
	pub fn new(e_type: EventType) -> Event {
		Event {
			handled: false,
			e_type,
		}
	}
	pub fn is_handled(&self) -> bool {
		self.handled
	}
}

impl From<EventType> for Event {
	fn from(e_type: EventType) -> Event {
		Event::new(e_type)
	}
}

impl<T> std::ops::Shl<T> for &Event
where
	Event: From<T>,
{
	type Output = bool;
	fn shl(self, rhs: T) -> bool {
		self.e_type == Event::from(rhs).e_type
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum EventType {
	WindowClosed,
	WindowResized(u32, u32),
	KeyPressed(Button, bool),
	KeyReleased(Button),
	CharWritten(char),
	MousePressed(Button),
	MouseReleased(Button),
	MouseMoved(Vec2),
	MouseScrolled(Vec2),
}
