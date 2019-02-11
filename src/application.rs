use crate::error::Error;
use crate::events::{self, EventListener};
use crate::layers::{Layer, LayerStack};
use crate::window;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Application {
	is_running: bool,
	window: window::Window,
	layer_stack: LayerStack,
}

static APPLICATION_EXISTS: AtomicBool = AtomicBool::new(false);
impl Application {
	pub fn new(title: &str, width: u32, height: u32, vsync: bool) -> Result<Application, Error> {
		if APPLICATION_EXISTS.swap(true, Ordering::Relaxed) {
			return Err(Error {});
		}
		
		let application = Application {
			is_running: true,
			window: window::Window::new(title, width, height, vsync)?,
			layer_stack: LayerStack::new(),
		};

		Ok(application)
	}

	pub fn start(mut self) {
		while self.is_running {
			self.window.on_update();
			while let Some(mut event) = self.window.pop_event() {
				event.dispatch(&mut self);
				for layer in self.layer_stack.iter_mut().rev() {
					event.dispatch(layer.as_event_listener());
					if event.is_handled() {
						break;
					}
				}
			}

			let mut event = events::AppUpdateEvent::new();
			for layer in self.layer_stack.iter_mut() {
				event.dispatch(layer.as_event_listener());
			}

			self.window.clear_screen();

			let mut event = events::AppRenderEvent::new();
			for layer in self.layer_stack.iter_mut() {
				event.dispatch(layer.as_event_listener());
			}

			self.window.swap_buffers().unwrap();
		}
	}

	pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
		self.layer_stack.push_layer(layer);
	}

	pub fn push_overlay(&mut self, layer: Box<dyn Layer>) {
		self.layer_stack.push_overlay(layer);
	}
}

impl EventListener for Application {
	fn on_window_closed(&mut self) {
		self.running = false;
	}
}
