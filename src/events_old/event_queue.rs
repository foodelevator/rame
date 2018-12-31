use std::collections::VecDeque;
use super::Event;

pub struct EventQueue {
	queue: VecDeque<Event>,
}

impl EventQueue {
	pub fn new() -> EventQueue {
		EventQueue {
			queue: VecDeque::new(),
		}
	}
	pub fn push_back<T>(&mut self, value: T)
	where
		Event: From<T>,
	{
		self.queue.push_back(Event::from(value));
	}
	pub fn pop_front(&mut self) -> Option<Event> {
		self.queue.pop_front()
	}
}
