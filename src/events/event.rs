use super::EventListener;

pub trait Event {
	fn is_handled(&self) -> bool;
	fn dispatch(&mut self, listener: &mut EventListener);
}
