use super::EventBox;
use std::collections::VecDeque;

pub struct EventQueue {
	queue: VecDeque<EventBox>,
}

impl EventQueue {
	pub fn new() -> EventQueue {
		EventQueue {
			queue: VecDeque::new(),
		}
	}
	pub fn push(&mut self, value: EventBox) {
		self.queue.push_back(value);
	}
	pub fn next(&mut self) -> Option<EventBox> {
		self.queue.pop_front()
	}
}
