use crate::error::Error;
use crate::events::{self, EventBox, EventQueue};
use crate::vecs::Vec2;
use crate::Float;

pub struct Window {
	title: String,
	width: u32,
	height: u32,
	vsync: bool,
	event_queue: EventQueue,
	// glutin specific stuff
	glutin_window: glutin::GlWindow,
	glutin_events: glutin::EventsLoop,
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
		let glutin_events = glutin::EventsLoop::new();
		let window_builder = glutin::WindowBuilder::new()
			.with_dimensions(glutin::dpi::LogicalSize::new(width as _, height as _))
			.with_title(title);
		let gl_context = glutin::ContextBuilder::new().
			with_vsync(vsync);

		let glutin_window = match glutin::GlWindow::new(window_builder, gl_context, &glutin_events) {
			Ok(w) => w,
			Err(_err) => return Err(Error {})
		};

		unsafe {
			use glutin::GlContext;
			glutin_window.make_current().unwrap();
			gl::load_with(|s| glutin_window.get_proc_address(s) as _);
		}

		Ok(Window {
			title: title.to_string(),
			width,
			height,
			vsync,
			event_queue: EventQueue::new(),
			glutin_window,
			glutin_events,
		})
	}

	pub fn set_vsync(&mut self, vsync: bool) {
		self.vsync = vsync;
		unimplemented!();
	}

	pub fn on_update(&mut self) {
		use crate::input;

		let mut event_queue = EventQueue::new();
		let mut new_size: Option<(u32, u32)> = None;
		self.glutin_events.poll_events(|event| {
			fn convert_key_event(input: glutin::KeyboardInput) -> input::Button {
				use crate::input::Button::*;
				let key = match input.virtual_keycode {
					Some(key) => key,
					None => return Unknown,
				};
				match key {
					glutin::VirtualKeyCode::Space => Space,
					glutin::VirtualKeyCode::Apostrophe => Apostrophe,
					glutin::VirtualKeyCode::Comma => Comma,
					glutin::VirtualKeyCode::Period => Period,
					glutin::VirtualKeyCode::Slash => Slash,
					glutin::VirtualKeyCode::Grave => Tilde,
					glutin::VirtualKeyCode::Key1 => Num1,
					glutin::VirtualKeyCode::Key2 => Num2,
					glutin::VirtualKeyCode::Key3 => Num3,
					glutin::VirtualKeyCode::Key4 => Num4,
					glutin::VirtualKeyCode::Key5 => Num5,
					glutin::VirtualKeyCode::Key6 => Num6,
					glutin::VirtualKeyCode::Key7 => Num7,
					glutin::VirtualKeyCode::Key8 => Num8,
					glutin::VirtualKeyCode::Key9 => Num9,
					glutin::VirtualKeyCode::Key0 => Num0,
					glutin::VirtualKeyCode::Subtract => Minus,
					glutin::VirtualKeyCode::Equals => Equals,
					glutin::VirtualKeyCode::A => A,
					glutin::VirtualKeyCode::B => B,
					glutin::VirtualKeyCode::C => C,
					glutin::VirtualKeyCode::D => D,
					glutin::VirtualKeyCode::E => E,
					glutin::VirtualKeyCode::F => F,
					glutin::VirtualKeyCode::G => G,
					glutin::VirtualKeyCode::H => H,
					glutin::VirtualKeyCode::I => I,
					glutin::VirtualKeyCode::J => J,
					glutin::VirtualKeyCode::K => K,
					glutin::VirtualKeyCode::L => L,
					glutin::VirtualKeyCode::M => M,
					glutin::VirtualKeyCode::N => N,
					glutin::VirtualKeyCode::O => O,
					glutin::VirtualKeyCode::P => P,
					glutin::VirtualKeyCode::Q => Q,
					glutin::VirtualKeyCode::R => R,
					glutin::VirtualKeyCode::S => S,
					glutin::VirtualKeyCode::T => T,
					glutin::VirtualKeyCode::U => U,
					glutin::VirtualKeyCode::V => V,
					glutin::VirtualKeyCode::W => W,
					glutin::VirtualKeyCode::X => X,
					glutin::VirtualKeyCode::Y => Y,
					glutin::VirtualKeyCode::Z => Z,
					glutin::VirtualKeyCode::LBracket => BracketLeft,
					glutin::VirtualKeyCode::RBracket => BracketRight,
					glutin::VirtualKeyCode::Backslash => Backslash,
					glutin::VirtualKeyCode::Semicolon => Semicolon,
					glutin::VirtualKeyCode::Escape => Esc,
					glutin::VirtualKeyCode::Return => Enter,
					glutin::VirtualKeyCode::Tab => Tab,
					glutin::VirtualKeyCode::Back => Backspace,
					glutin::VirtualKeyCode::Insert => Insert,
					glutin::VirtualKeyCode::Delete => Delete,
					glutin::VirtualKeyCode::Right => ArrowRight,
					glutin::VirtualKeyCode::Left => ArrowLeft,
					glutin::VirtualKeyCode::Down => ArrowDown,
					glutin::VirtualKeyCode::Up => ArrowUp,
					glutin::VirtualKeyCode::PageUp => PgUp,
					glutin::VirtualKeyCode::PageDown => PgDown,
					glutin::VirtualKeyCode::Home => Home,
					glutin::VirtualKeyCode::End => End,
					glutin::VirtualKeyCode::Capital => CapsLock,
					glutin::VirtualKeyCode::Scroll => ScrollLock,
					glutin::VirtualKeyCode::Numlock => NumLock,
					glutin::VirtualKeyCode::Snapshot => PrintScreen,
					glutin::VirtualKeyCode::Pause => Pause,
					glutin::VirtualKeyCode::F1 => F1,
					glutin::VirtualKeyCode::F2 => F2,
					glutin::VirtualKeyCode::F3 => F3,
					glutin::VirtualKeyCode::F4 => F4,
					glutin::VirtualKeyCode::F5 => F5,
					glutin::VirtualKeyCode::F6 => F6,
					glutin::VirtualKeyCode::F7 => F7,
					glutin::VirtualKeyCode::F8 => F8,
					glutin::VirtualKeyCode::F9 => F9,
					glutin::VirtualKeyCode::F10 => F10,
					glutin::VirtualKeyCode::F11 => F11,
					glutin::VirtualKeyCode::F12 => F12,
					glutin::VirtualKeyCode::Numpad0 => NumPad0,
					glutin::VirtualKeyCode::Numpad1 => NumPad1,
					glutin::VirtualKeyCode::Numpad2 => NumPad2,
					glutin::VirtualKeyCode::Numpad3 => NumPad3,
					glutin::VirtualKeyCode::Numpad4 => NumPad4,
					glutin::VirtualKeyCode::Numpad5 => NumPad5,
					glutin::VirtualKeyCode::Numpad6 => NumPad6,
					glutin::VirtualKeyCode::Numpad7 => NumPad7,
					glutin::VirtualKeyCode::Numpad8 => NumPad8,
					glutin::VirtualKeyCode::Numpad9 => NumPad9,
					glutin::VirtualKeyCode::NumpadComma => NumPadDec,
					glutin::VirtualKeyCode::Divide => NumPadDiv,
					glutin::VirtualKeyCode::Multiply => NumPadMult,
					glutin::VirtualKeyCode::Minus => NumPadSub,
					glutin::VirtualKeyCode::Add => NumPadAdd,
					glutin::VirtualKeyCode::NumpadEnter => NumPadEnter,
					glutin::VirtualKeyCode::NumpadEquals => NumPadEq,
					glutin::VirtualKeyCode::LShift => LShift,
					glutin::VirtualKeyCode::LControl => LCtrl,
					glutin::VirtualKeyCode::LAlt => LAlt,
					glutin::VirtualKeyCode::LWin => LSuper,
					glutin::VirtualKeyCode::RShift => RShift,
					glutin::VirtualKeyCode::RControl => RCtrl,
					glutin::VirtualKeyCode::RAlt => RAlt,
					glutin::VirtualKeyCode::RWin => RSuper,
					_ => Unknown,
				}
			}
			match event {
				glutin::Event::WindowEvent { event, .. } => {
					match event {
						glutin::WindowEvent::CloseRequested => {
							event_queue.push(events::WindowClosedEvent::new());
						}
						glutin::WindowEvent::Resized(size) => {
							let (width, height): (u32, u32) = size.into();
							new_size = Some((width, height));
							event_queue.push(events::WindowResizedEvent::new(width, height));
						}
						glutin::WindowEvent::KeyboardInput { input, .. } => {
							event_queue.push(match input.state {
								glutin::ElementState::Pressed => {
									events::KeyPressedEvent::new(convert_key_event(input), false)
								}
								// TODO: add key repetition detection
								glutin::ElementState::Released => {
									events::KeyReleasedEvent::new(convert_key_event(input))
								}
							});
						}
						glutin::WindowEvent::ReceivedCharacter(c) => {
							event_queue.push(events::CharWrittenEvent::new(c));
						}
						glutin::WindowEvent::MouseInput { state, button, .. } => {
							event_queue.push(match state {
								glutin::ElementState::Pressed => {
									events::MousePressedEvent::new(match button {
										glutin::MouseButton::Left => input::Button::MouseLeft,
										glutin::MouseButton::Middle => input::Button::MouseMiddle,
										glutin::MouseButton::Right => input::Button::MouseRight,
										glutin::MouseButton::Other(4) => input::Button::Mouse4,
										glutin::MouseButton::Other(5) => input::Button::Mouse5,
										glutin::MouseButton::Other(6) => input::Button::Mouse6,
										glutin::MouseButton::Other(7) => input::Button::Mouse7,
										glutin::MouseButton::Other(8) => input::Button::Mouse8,
										glutin::MouseButton::Other(_) => input::Button::Unknown,
									})
								}
								glutin::ElementState::Released => {
									events::MouseReleasedEvent::new(match button {
										glutin::MouseButton::Left => input::Button::MouseLeft,
										glutin::MouseButton::Middle => input::Button::MouseMiddle,
										glutin::MouseButton::Right => input::Button::MouseRight,
										glutin::MouseButton::Other(4) => input::Button::Mouse4,
										glutin::MouseButton::Other(5) => input::Button::Mouse5,
										glutin::MouseButton::Other(6) => input::Button::Mouse6,
										glutin::MouseButton::Other(7) => input::Button::Mouse7,
										glutin::MouseButton::Other(8) => input::Button::Mouse8,
										glutin::MouseButton::Other(_) => input::Button::Unknown,
									})
								}
							});
						}
						glutin::WindowEvent::MouseWheel { delta, .. } => {
							event_queue.push(events::MouseScrolledEvent::new(match delta {
								glutin::MouseScrollDelta::LineDelta(x, y) => Vec2::new(x.into(), y.into()),
								glutin::MouseScrollDelta::PixelDelta(d) => Vec2::new(d.x as Float, d.y as Float),
								// NOTE: Perhaps scrolling speed will be fucked
								// depending on which eventis received here.
							}));
						}
						glutin::WindowEvent::CursorMoved { position: pos, .. } => {
							event_queue.push(events::MouseMovedEvent::new(Vec2::new(pos.x as Float, pos.y as Float)));
						}
						_ => {}
					}
				},
				_ => {}
			}
		});

		if let Some(new_size) = new_size {
			self.width = new_size.0;
			self.height = new_size.1;
		}

		while let Some(event) = event_queue.next() {
			self.event_queue.push(event);
		}
	}

	pub fn clear_color(&mut self, r: Float, g: Float, b: Float) {
		unsafe {
			gl::ClearColor(r as _, g as _, b as _, 1.0);
		}
	}

	pub fn clear_screen(&mut self) {
		unsafe {
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}
	}

	pub fn swap_buffers(&mut self) -> Result<(), Error> {
		match self.glutin_window.swap_buffers() {
			Ok(()) => Ok(()),
			Err(_err) => Err(Error {}),
		}
	}
}
