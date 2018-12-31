use super::Layer;
use std::collections::{vec_deque::IterMut, VecDeque};

pub struct LayerStack {
	layers: VecDeque<Box<dyn Layer>>,
}

impl LayerStack {
	pub fn new() -> LayerStack {
		LayerStack {
			layers: VecDeque::new(),
		}
	}
	pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
		self.layers.push_front(layer);
		self.layers.front_mut().unwrap().on_attach();
	}
	pub fn push_overlay(&mut self, layer: Box<dyn Layer>) {
		self.layers.push_back(layer);
		self.layers.back_mut().unwrap().on_attach();
	}
	pub fn pop_layer(&mut self) {
		if let Some(mut layer) = self.layers.pop_front() {
			layer.on_detach();
		}
	}
	pub fn pop_overlay(&mut self) {
		if let Some(mut layer) = self.layers.pop_back() {
			layer.on_detach();
		}
	}
	pub fn iter_mut(&mut self) -> IterMut<Box<dyn Layer>> {
		self.layers.iter_mut()
	}
}

impl Drop for LayerStack {
	fn drop(&mut self) {
		for layer in self.iter_mut() {
			layer.on_detach();
		}
	}
}
