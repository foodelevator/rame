use crate::error::Error;
use crate::events::{self, EventBox, EventQueue};
use crate::vecs::Vec2;

pub struct Window {
	title: String,
	width: u32,
	height: u32,
	vsync: bool,
	event_queue: EventQueue,
	// winit specific stuff
	winit_window: winit::Window,
	winit_events: winit::EventsLoop,
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
		let winit_events = winit::EventsLoop::new();
		let winit_window = match winit::WindowBuilder::new()
			.with_title(title)
			.build(&winit_events) {
			Ok(winit_window) => winit_window,
			Err(_) => return Err(Error {}),
		};

		// load gl functions

		// set vsync

		Ok(Window {
			title: title.to_string(),
			width,
			height,
			vsync,
			event_queue: EventQueue::new(),
			winit_window,
			winit_events,
		})
	}

	pub fn set_vsync(&mut self, vsync: bool) {
		self.vsync = vsync;
		// set vsync
	}

	pub fn on_update(&mut self) {
		use crate::input;

		let mut event_queue = EventQueue::new();
		let mut new_size: Option<(u32, u32)> = None;
		self.winit_events.poll_events(|event| {
			fn convert_key_event(input: winit::KeyboardInput) -> input::Button {
				use crate::input::Button::*;
				let key = match input.virtual_keycode {
					Some(key) => key,
					None => return Unknown,
				};
				match key {
					winit::VirtualKeyCode::Space => Space,
					winit::VirtualKeyCode::Apostrophe => Apostrophe,
					winit::VirtualKeyCode::Comma => Comma,
					winit::VirtualKeyCode::Period => Period,
					winit::VirtualKeyCode::Slash => Slash,
					winit::VirtualKeyCode::Grave => Tilde,
					winit::VirtualKeyCode::Key1 => Num1,
					winit::VirtualKeyCode::Key2 => Num2,
					winit::VirtualKeyCode::Key3 => Num3,
					winit::VirtualKeyCode::Key4 => Num4,
					winit::VirtualKeyCode::Key5 => Num5,
					winit::VirtualKeyCode::Key6 => Num6,
					winit::VirtualKeyCode::Key7 => Num7,
					winit::VirtualKeyCode::Key8 => Num8,
					winit::VirtualKeyCode::Key9 => Num9,
					winit::VirtualKeyCode::Key0 => Num0,
					winit::VirtualKeyCode::Subtract => Minus,
					winit::VirtualKeyCode::Equals => Equals,
					winit::VirtualKeyCode::A => A,
					winit::VirtualKeyCode::B => B,
					winit::VirtualKeyCode::C => C,
					winit::VirtualKeyCode::D => D,
					winit::VirtualKeyCode::E => E,
					winit::VirtualKeyCode::F => F,
					winit::VirtualKeyCode::G => G,
					winit::VirtualKeyCode::H => H,
					winit::VirtualKeyCode::I => I,
					winit::VirtualKeyCode::J => J,
					winit::VirtualKeyCode::K => K,
					winit::VirtualKeyCode::L => L,
					winit::VirtualKeyCode::M => M,
					winit::VirtualKeyCode::N => N,
					winit::VirtualKeyCode::O => O,
					winit::VirtualKeyCode::P => P,
					winit::VirtualKeyCode::Q => Q,
					winit::VirtualKeyCode::R => R,
					winit::VirtualKeyCode::S => S,
					winit::VirtualKeyCode::T => T,
					winit::VirtualKeyCode::U => U,
					winit::VirtualKeyCode::V => V,
					winit::VirtualKeyCode::W => W,
					winit::VirtualKeyCode::X => X,
					winit::VirtualKeyCode::Y => Y,
					winit::VirtualKeyCode::Z => Z,
					winit::VirtualKeyCode::LBracket => BracketLeft,
					winit::VirtualKeyCode::RBracket => BracketRight,
					winit::VirtualKeyCode::Backslash => Backslash,
					winit::VirtualKeyCode::Semicolon => Semicolon,
					winit::VirtualKeyCode::Escape => Esc,
					winit::VirtualKeyCode::Return => Enter,
					winit::VirtualKeyCode::Tab => Tab,
					winit::VirtualKeyCode::Back => Backspace,
					winit::VirtualKeyCode::Insert => Insert,
					winit::VirtualKeyCode::Delete => Delete,
					winit::VirtualKeyCode::Right => ArrowRight,
					winit::VirtualKeyCode::Left => ArrowLeft,
					winit::VirtualKeyCode::Down => ArrowDown,
					winit::VirtualKeyCode::Up => ArrowUp,
					winit::VirtualKeyCode::PageUp => PgUp,
					winit::VirtualKeyCode::PageDown => PgDown,
					winit::VirtualKeyCode::Home => Home,
					winit::VirtualKeyCode::End => End,
					winit::VirtualKeyCode::Capital => CapsLock,
					winit::VirtualKeyCode::Scroll => ScrollLock,
					winit::VirtualKeyCode::Numlock => NumLock,
					winit::VirtualKeyCode::Snapshot => PrintScreen,
					winit::VirtualKeyCode::Pause => Pause,
					winit::VirtualKeyCode::F1 => F1,
					winit::VirtualKeyCode::F2 => F2,
					winit::VirtualKeyCode::F3 => F3,
					winit::VirtualKeyCode::F4 => F4,
					winit::VirtualKeyCode::F5 => F5,
					winit::VirtualKeyCode::F6 => F6,
					winit::VirtualKeyCode::F7 => F7,
					winit::VirtualKeyCode::F8 => F8,
					winit::VirtualKeyCode::F9 => F9,
					winit::VirtualKeyCode::F10 => F10,
					winit::VirtualKeyCode::F11 => F11,
					winit::VirtualKeyCode::F12 => F12,
					// there are up to F25 in glfw but fuk dat
					winit::VirtualKeyCode::Numpad0 => NumPad0,
					winit::VirtualKeyCode::Numpad1 => NumPad1,
					winit::VirtualKeyCode::Numpad2 => NumPad2,
					winit::VirtualKeyCode::Numpad3 => NumPad3,
					winit::VirtualKeyCode::Numpad4 => NumPad4,
					winit::VirtualKeyCode::Numpad5 => NumPad5,
					winit::VirtualKeyCode::Numpad6 => NumPad6,
					winit::VirtualKeyCode::Numpad7 => NumPad7,
					winit::VirtualKeyCode::Numpad8 => NumPad8,
					winit::VirtualKeyCode::Numpad9 => NumPad9,
					winit::VirtualKeyCode::NumpadComma => NumPadDec,
					winit::VirtualKeyCode::Divide => NumPadDiv,
					winit::VirtualKeyCode::Multiply => NumPadMult,
					winit::VirtualKeyCode::Minus => NumPadSub,
					winit::VirtualKeyCode::Add => NumPadAdd,
					winit::VirtualKeyCode::NumpadEnter => NumPadEnter,
					winit::VirtualKeyCode::NumpadEquals => NumPadEq,
					winit::VirtualKeyCode::LShift => LShift,
					winit::VirtualKeyCode::LControl => LCtrl,
					winit::VirtualKeyCode::LAlt => LAlt,
					winit::VirtualKeyCode::LWin => LSuper,
					winit::VirtualKeyCode::RShift => RShift,
					winit::VirtualKeyCode::RControl => RCtrl,
					winit::VirtualKeyCode::RAlt => RAlt,
					winit::VirtualKeyCode::RWin => RSuper,
					_ => Unknown,
				}
			}
			match event {
				winit::Event::WindowEvent { event, .. } => {
					match event {
						winit::WindowEvent::CloseRequested => {
							event_queue.push(events::WindowClosedEvent::new());
						}
						winit::WindowEvent::Resized(size) => {
							let (width, height): (u32, u32) = size.into();
							new_size = Some((width, height));
							event_queue.push(events::WindowResizedEvent::new(width, height));
						}
						winit::WindowEvent::KeyboardInput { input, .. } => {
							event_queue.push(match input.state {
								winit::ElementState::Pressed => {
									events::KeyPressedEvent::new(convert_key_event(input), false)
								}
								// TODO: add key repetition detection
								winit::ElementState::Released => {
									events::KeyReleasedEvent::new(convert_key_event(input))
								}
							});
						}
						winit::WindowEvent::ReceivedCharacter(c) => {
							event_queue.push(events::CharWrittenEvent::new(c));
						}
						winit::WindowEvent::MouseInput { state, button, .. } => {
							event_queue.push(match state {
								winit::ElementState::Pressed => {
									events::MousePressedEvent::new(match button {
										winit::MouseButton::Left => input::Button::MouseLeft,
										winit::MouseButton::Middle => input::Button::MouseMiddle,
										winit::MouseButton::Right => input::Button::MouseRight,
										winit::MouseButton::Other(4) => input::Button::Mouse4,
										winit::MouseButton::Other(5) => input::Button::Mouse5,
										winit::MouseButton::Other(6) => input::Button::Mouse6,
										winit::MouseButton::Other(7) => input::Button::Mouse7,
										winit::MouseButton::Other(8) => input::Button::Mouse8,
										winit::MouseButton::Other(_) => input::Button::Unknown,
									})
								}
								winit::ElementState::Released => {
									events::MouseReleasedEvent::new(match button {
										winit::MouseButton::Left => input::Button::MouseLeft,
										winit::MouseButton::Middle => input::Button::MouseMiddle,
										winit::MouseButton::Right => input::Button::MouseRight,
										winit::MouseButton::Other(4) => input::Button::Mouse4,
										winit::MouseButton::Other(5) => input::Button::Mouse5,
										winit::MouseButton::Other(6) => input::Button::Mouse6,
										winit::MouseButton::Other(7) => input::Button::Mouse7,
										winit::MouseButton::Other(8) => input::Button::Mouse8,
										winit::MouseButton::Other(_) => input::Button::Unknown,
									})
								}
							});
						}
						winit::WindowEvent::MouseWheel { delta, .. } => {
							event_queue.push(events::MouseScrolledEvent::new(match delta {
								winit::MouseScrollDelta::LineDelta(x, y) => Vec2::new(x.into(), y.into()),
								winit::MouseScrollDelta::PixelDelta(d) => Vec2::new(d.x, d.y),
								// NOTE: Perhaps scrolling speed will be fucked
								// depending on which eventis received here.
							}));
						}
						winit::WindowEvent::CursorMoved { position: pos, .. } => {
							event_queue.push(events::MouseMovedEvent::new(Vec2::new(pos.x, pos.y)));
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

	pub fn clear_screen(&mut self) {
		unsafe {
			// gl::ClearColor(0.05, 0.05, 0.09, 1.0);
			// gl::Clear(gl::COLOR_BUFFER_BIT);
		}
	}

	pub fn swap_buffers(&mut self) {
		// is this even possible with winit?
	}
}
