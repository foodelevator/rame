use crate::error::Error;
use crate::events::{self, EventBox, EventQueue};
use crate::vecs::Vec2;
use std::sync::mpsc;

pub struct Window {
	title: String,
	width: u32,
	height: u32,
	vsync: bool,
	event_queue: EventQueue,
	// glfw specific stuff
	glfw_window: glfw::Window,
	glfw_events: mpsc::Receiver<(f64, glfw::WindowEvent)>,
}

impl Window {
	pub fn get_title(&self) -> &str {
		&self.title
	}

	pub fn get_width(&self) -> u32 {
		self.width
	}

	pub fn get_height(&self) -> u32 {
		self.height
	}

	pub fn get_vsync(&self) -> bool {
		self.vsync
	}

	pub fn pop_event(&mut self) -> Option<EventBox> {
		self.event_queue.next()
	}
}

// glfw specific stuff
impl Window {
	pub fn new(title: &str, width: u32, height: u32, vsync: bool) -> Result<Window, Error> {
		let mut glfw = match glfw::init(glfw::FAIL_ON_ERRORS) {
			Ok(glfw) => glfw,
			Err(glfw::InitError::AlreadyInitialized) => glfw::Glfw,
			Err(_) => return Err(Error {}), // TODO: add info
		};

		let (mut glfw_window, glfw_events) =
			match glfw.create_window(width, height, title, glfw::WindowMode::Windowed) {
				Some(w) => w,
				None => return Err(Error {}), // TODO: add info
			};

		// Don't load the functions again if another window is created
		if !gl::ClearColor::is_loaded() {
			gl::load_with(|s| glfw_window.get_proc_address(s));
		} else {
			use glfw::Context;
			glfw_window.make_current();
		}

		glfw_window.set_all_polling(true); // TODO: perhaps dont listen to EVERYTHING all the time

		glfw.set_swap_interval(if vsync {
			glfw::SwapInterval::Sync(1)
		} else {
			glfw::SwapInterval::None
		});

		Ok(Window {
			title: title.to_string(),
			width,
			height,
			vsync,
			event_queue: EventQueue::new(),
			glfw_window,
			glfw_events,
		})
	}

	pub fn set_vsync(&mut self, vsync: bool) {
		self.vsync = vsync;
		self.glfw_window.glfw.set_swap_interval(if vsync {
			glfw::SwapInterval::Sync(1)
		} else {
			glfw::SwapInterval::None
		});
	}

	pub fn on_update(&mut self) {
		use crate::input;

		self.glfw_window.glfw.poll_events();
		for (_time, event) in glfw::flush_messages(&self.glfw_events) {
			fn convert_key_event(key: glfw::Key) -> input::Button {
				use crate::input::Button::*;
				use glfw::Key;
				match key {
					Key::Space => Space,
					Key::Apostrophe => Apostrophe,
					Key::Comma => Comma,
					Key::Minus => Minus,
					Key::Period => Period,
					Key::Slash => Slash,
					Key::Num0 => Num0,
					Key::Num1 => Num1,
					Key::Num2 => Num2,
					Key::Num3 => Num3,
					Key::Num4 => Num4,
					Key::Num5 => Num5,
					Key::Num6 => Num6,
					Key::Num7 => Num7,
					Key::Num8 => Num8,
					Key::Num9 => Num9,
					Key::Semicolon => Semicolon,
					Key::Equal => Equals,
					Key::A => A,
					Key::B => B,
					Key::C => C,
					Key::D => D,
					Key::E => E,
					Key::F => F,
					Key::G => G,
					Key::H => H,
					Key::I => I,
					Key::J => J,
					Key::K => K,
					Key::L => L,
					Key::M => M,
					Key::N => N,
					Key::O => O,
					Key::P => P,
					Key::Q => Q,
					Key::R => R,
					Key::S => S,
					Key::T => T,
					Key::U => U,
					Key::V => V,
					Key::W => W,
					Key::X => X,
					Key::Y => Y,
					Key::Z => Z,
					Key::LeftBracket => BracketLeft,
					Key::Backslash => Backslash,
					Key::RightBracket => BracketRight,
					Key::GraveAccent => Tilde,
					Key::Escape => Esc,
					Key::Enter => Enter,
					Key::Tab => Tab,
					Key::Backspace => Backspace,
					Key::Insert => Insert,
					Key::Delete => Delete,
					Key::Right => ArrowRight,
					Key::Left => ArrowLeft,
					Key::Down => ArrowDown,
					Key::Up => ArrowUp,
					Key::PageUp => PgUp,
					Key::PageDown => PgDown,
					Key::Home => Home,
					Key::End => End,
					Key::CapsLock => CapsLock,
					Key::ScrollLock => ScrollLock,
					Key::NumLock => NumLock,
					Key::PrintScreen => PrintScreen,
					Key::Pause => Pause,
					Key::F1 => F1,
					Key::F2 => F2,
					Key::F3 => F3,
					Key::F4 => F4,
					Key::F5 => F5,
					Key::F6 => F6,
					Key::F7 => F7,
					Key::F8 => F8,
					Key::F9 => F9,
					Key::F10 => F10,
					Key::F11 => F11,
					Key::F12 => F12,
					// there are up to F25 in glfw but fuk dat
					Key::Kp0 => NumPad0,
					Key::Kp1 => NumPad1,
					Key::Kp2 => NumPad2,
					Key::Kp3 => NumPad3,
					Key::Kp4 => NumPad4,
					Key::Kp5 => NumPad5,
					Key::Kp6 => NumPad6,
					Key::Kp7 => NumPad7,
					Key::Kp8 => NumPad8,
					Key::Kp9 => NumPad9,
					Key::KpDecimal => NumPadDec,
					Key::KpDivide => NumPadDiv,
					Key::KpMultiply => NumPadMult,
					Key::KpSubtract => NumPadSub,
					Key::KpAdd => NumPadAdd,
					Key::KpEnter => NumPadEnter,
					Key::KpEqual => NumPadEq,
					Key::LeftShift => LShift,
					Key::LeftControl => LCtrl,
					Key::LeftAlt => LAlt,
					Key::LeftSuper => LSuper,
					Key::RightShift => RShift,
					Key::RightControl => RCtrl,
					Key::RightAlt => RAlt,
					Key::RightSuper => RSuper,
					Key::Menu => Menu,
					_ => Unknown,
				}
			}
			match event {
				glfw::WindowEvent::Close => {
					self.event_queue.push(events::WindowClosedEvent::new());
				}
				glfw::WindowEvent::Size(width, height) => {
					self.width = width as u32;
					self.height = height as u32;
					self.event_queue
						.push(events::WindowResizedEvent::new(width as u32, height as u32));
				}
				glfw::WindowEvent::Key(key, _, glfw::Action::Press, _) => {
					self.event_queue
						.push(events::KeyPressedEvent::new(convert_key_event(key), false));
				}
				glfw::WindowEvent::Key(key, _, glfw::Action::Repeat, _) => {
					self.event_queue
						.push(events::KeyPressedEvent::new(convert_key_event(key), true));
				}
				glfw::WindowEvent::Key(key, _, glfw::Action::Release, _) => {
					self.event_queue
						.push(events::KeyReleasedEvent::new(convert_key_event(key)));
				}
				glfw::WindowEvent::Char(c) => {
					self.event_queue.push(events::CharWrittenEvent::new(c));
				}
				glfw::WindowEvent::MouseButton(button, glfw::Action::Press, _) => {
					self.event_queue
						.push(events::MousePressedEvent::new(match button {
							glfw::MouseButton::Button1 => input::Button::MouseLeft,
							glfw::MouseButton::Button2 => input::Button::MouseRight,
							glfw::MouseButton::Button3 => input::Button::MouseMiddle,
							glfw::MouseButton::Button4 => input::Button::Mouse4,
							glfw::MouseButton::Button5 => input::Button::Mouse5,
							glfw::MouseButton::Button6 => input::Button::Mouse6,
							glfw::MouseButton::Button7 => input::Button::Mouse7,
							glfw::MouseButton::Button8 => input::Button::Mouse8,
						}));
				}
				glfw::WindowEvent::MouseButton(button, glfw::Action::Release, _) => {
					self.event_queue
						.push(events::MouseReleasedEvent::new(match button {
							glfw::MouseButton::Button1 => input::Button::MouseLeft,
							glfw::MouseButton::Button2 => input::Button::MouseRight,
							glfw::MouseButton::Button3 => input::Button::MouseMiddle,
							glfw::MouseButton::Button4 => input::Button::Mouse4,
							glfw::MouseButton::Button5 => input::Button::Mouse5,
							glfw::MouseButton::Button6 => input::Button::Mouse6,
							glfw::MouseButton::Button7 => input::Button::Mouse7,
							glfw::MouseButton::Button8 => input::Button::Mouse8,
						}));
				}
				glfw::WindowEvent::Scroll(delta_x, delta_y) => {
					self.event_queue
						.push(events::MouseScrolledEvent::new(Vec2::new(delta_x, delta_y)));
				}
				glfw::WindowEvent::CursorPos(x, y) => {
					self.event_queue
						.push(events::MouseMovedEvent::new(Vec2::new(x, y)));
				}
				_ => {}
			}
		}
	}

	pub fn clear_screen(&mut self) {
		unsafe {
			gl::ClearColor(0.05, 0.05, 0.09, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}
	}

	pub fn swap_buffers(&mut self) {
		use glfw::Context;
		self.glfw_window.swap_buffers();
	}
}
