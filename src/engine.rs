use crate::error::Error;
use crate::application::Application;
use crate::events::EventListener;
use crate::input::{Button, INPUT_STATE};
use crate::layers::{Layer, LayerStack};
use crate::vecs::Vec2;
use crate::window::{Window, WindowOptions};

pub struct Engine<A> {
	is_running: bool,
	window: Window,
	layer_stack: LayerStack,
	app: A,
}

impl<A> Engine<A>
where A: Application
{
	pub fn init<W>(app: A, window_opts: W) -> Result<(), Error>
	where W: Into<WindowOptions> {

		let mut engine = Engine {
			is_running: true,
			window: Window::new(window_opts.into())?,
			layer_stack: LayerStack::new(),
			app
		};

		engine.main_loop()?;

		Ok(())
	}

	fn main_loop(&mut self) -> Result<(), Error> {
		while self.is_running {
			for mut event in self.window.on_update() {
				event.dispatch(self);
				for layer in self.layer_stack.iter_mut().rev() {
					event.dispatch(layer.as_event_listener());
					if event.is_handled() {
						break;
					}
				}
			}

			for layer in self.layer_stack.iter_mut().rev() {
				layer.on_update();
			}

			self.window.clear_screen();

			for layer in self.layer_stack.iter_mut() {
				layer.on_render();
			}

			self.window.swap_buffers().unwrap();
		}
		
		Ok(())
	}

	pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
		self.layer_stack.push_layer(layer);
	}

	pub fn push_overlay(&mut self, layer: Box<dyn Layer>) {
		self.layer_stack.push_overlay(layer);
	}
}

impl<A> EventListener for Engine<A> {
	fn on_window_closed(&mut self) {
		self.is_running = false;
	}
	fn on_mouse_move(&mut self, position: Vec2) -> bool {
		unsafe {
			INPUT_STATE.mouse_position = position;
		}
		false
	}
	fn on_key_press(&mut self, button: Button, _: bool) -> bool {
		use Button::*;
		unsafe {
			match button {
				Unknown => {}
				MouseLeft => INPUT_STATE.mouse_left = true,
				MouseRight => INPUT_STATE.mouse_right = true,
				MouseMiddle => INPUT_STATE.mouse_middle = true,
				Mouse4 => INPUT_STATE.mouse4 = true,
				Mouse5 => INPUT_STATE.mouse5 = true,
				Mouse6 => INPUT_STATE.mouse6 = true,
				Mouse7 => INPUT_STATE.mouse7 = true,
				Mouse8 => INPUT_STATE.mouse8 = true,
				LShift => INPUT_STATE.l_shift = true,
				LCtrl => INPUT_STATE.l_ctrl = true,
				LAlt => INPUT_STATE.l_alt = true,
				LSuper => INPUT_STATE.l_super = true,
				RShift => INPUT_STATE.r_shift = true,
				RCtrl => INPUT_STATE.r_ctrl = true,
				RAlt => INPUT_STATE.r_alt = true,
				RSuper => INPUT_STATE.r_super = true,
				Tab => INPUT_STATE.tab = true,
				CapsLock => INPUT_STATE.caps_lock = true,
				Backspace => INPUT_STATE.backspace = true,
				Esc => INPUT_STATE.esc = true,
				F1 => INPUT_STATE.f1 = true,
				F2 => INPUT_STATE.f2 = true,
				F3 => INPUT_STATE.f3 = true,
				F4 => INPUT_STATE.f4 = true,
				F5 => INPUT_STATE.f5 = true,
				F6 => INPUT_STATE.f6 = true,
				F7 => INPUT_STATE.f7 = true,
				F8 => INPUT_STATE.f8 = true,
				F9 => INPUT_STATE.f9 = true,
				F10 => INPUT_STATE.f10 = true,
				F11 => INPUT_STATE.f11 = true,
				F12 => INPUT_STATE.f12 = true,
				PrintScreen => INPUT_STATE.print_screen = true,
				ScrollLock => INPUT_STATE.scroll_lock = true,
				Pause => INPUT_STATE.pause = true,
				Insert => INPUT_STATE.insert = true,
				Delete => INPUT_STATE.delete = true,
				Home => INPUT_STATE.home = true,
				End => INPUT_STATE.end = true,
				PgUp => INPUT_STATE.pg_up = true,
				PgDown => INPUT_STATE.pg_down = true,
				Menu => INPUT_STATE.menu = true,
				ArrowLeft => INPUT_STATE.arrow_left = true,
				ArrowUp => INPUT_STATE.arrow_up = true,
				ArrowRight => INPUT_STATE.arrow_right = true,
				ArrowDown => INPUT_STATE.arrow_down = true,
				Tilde => INPUT_STATE.tilde = true,
				Num1 => INPUT_STATE.num1 = true,
				Num2 => INPUT_STATE.num2 = true,
				Num3 => INPUT_STATE.num3 = true,
				Num4 => INPUT_STATE.num4 = true,
				Num5 => INPUT_STATE.num5 = true,
				Num6 => INPUT_STATE.num6 = true,
				Num7 => INPUT_STATE.num7 = true,
				Num8 => INPUT_STATE.num8 = true,
				Num9 => INPUT_STATE.num9 = true,
				Num0 => INPUT_STATE.num0 = true,
				Q => INPUT_STATE.q = true,
				W => INPUT_STATE.w = true,
				E => INPUT_STATE.e = true,
				R => INPUT_STATE.r = true,
				T => INPUT_STATE.t = true,
				Y => INPUT_STATE.y = true,
				U => INPUT_STATE.u = true,
				I => INPUT_STATE.i = true,
				O => INPUT_STATE.o = true,
				P => INPUT_STATE.p = true,
				A => INPUT_STATE.a = true,
				S => INPUT_STATE.s = true,
				D => INPUT_STATE.d = true,
				F => INPUT_STATE.f = true,
				G => INPUT_STATE.g = true,
				H => INPUT_STATE.h = true,
				J => INPUT_STATE.j = true,
				K => INPUT_STATE.k = true,
				L => INPUT_STATE.l = true,
				Z => INPUT_STATE.z = true,
				X => INPUT_STATE.x = true,
				C => INPUT_STATE.c = true,
				V => INPUT_STATE.v = true,
				B => INPUT_STATE.b = true,
				N => INPUT_STATE.n = true,
				M => INPUT_STATE.m = true,
				Space => INPUT_STATE.space = true,
				Minus => INPUT_STATE.minus = true,
				Equals => INPUT_STATE.equals = true,
				BracketLeft => INPUT_STATE.bracket_left = true,
				BracketRight => INPUT_STATE.bracket_right = true,
				Backslash => INPUT_STATE.backslash = true,
				Semicolon => INPUT_STATE.semicolon = true,
				Apostrophe => INPUT_STATE.apostrophe = true,
				Enter => INPUT_STATE.enter = true,
				Comma => INPUT_STATE.comma = true,
				Period => INPUT_STATE.period = true,
				Slash => INPUT_STATE.slash = true,
				NumPad0 => INPUT_STATE.numpad0 = true,
				NumPad1 => INPUT_STATE.numpad1 = true,
				NumPad2 => INPUT_STATE.numpad2 = true,
				NumPad3 => INPUT_STATE.numpad3 = true,
				NumPad4 => INPUT_STATE.numpad4 = true,
				NumPad5 => INPUT_STATE.numpad5 = true,
				NumPad6 => INPUT_STATE.numpad6 = true,
				NumPad7 => INPUT_STATE.numpad7 = true,
				NumPad8 => INPUT_STATE.numpad8 = true,
				NumPad9 => INPUT_STATE.numpad9 = true,
				NumLock => INPUT_STATE.numlock = true,
				NumPadDec => INPUT_STATE.numpad_dec = true,
				NumPadDiv => INPUT_STATE.numpad_div = true,
				NumPadMult => INPUT_STATE.numpad_mult = true,
				NumPadSub => INPUT_STATE.numpad_sub = true,
				NumPadAdd => INPUT_STATE.numpad_add = true,
				NumPadEq => INPUT_STATE.numpad_eq = true,
				NumPadEnter => INPUT_STATE.numpad_enter = true,
			}
		}
		false
	}
	fn on_key_release(&mut self, button: Button) -> bool {
		use Button::*;
		unsafe {
			match button {
				Unknown => {}
				MouseLeft => INPUT_STATE.mouse_left = false,
				MouseRight => INPUT_STATE.mouse_right = false,
				MouseMiddle => INPUT_STATE.mouse_middle = false,
				Mouse4 => INPUT_STATE.mouse4 = false,
				Mouse5 => INPUT_STATE.mouse5 = false,
				Mouse6 => INPUT_STATE.mouse6 = false,
				Mouse7 => INPUT_STATE.mouse7 = false,
				Mouse8 => INPUT_STATE.mouse8 = false,
				LShift => INPUT_STATE.l_shift = false,
				LCtrl => INPUT_STATE.l_ctrl = false,
				LAlt => INPUT_STATE.l_alt = false,
				LSuper => INPUT_STATE.l_super = false,
				RShift => INPUT_STATE.r_shift = false,
				RCtrl => INPUT_STATE.r_ctrl = false,
				RAlt => INPUT_STATE.r_alt = false,
				RSuper => INPUT_STATE.r_super = false,
				Tab => INPUT_STATE.tab = false,
				CapsLock => INPUT_STATE.caps_lock = false,
				Backspace => INPUT_STATE.backspace = false,
				Esc => INPUT_STATE.esc = false,
				F1 => INPUT_STATE.f1 = false,
				F2 => INPUT_STATE.f2 = false,
				F3 => INPUT_STATE.f3 = false,
				F4 => INPUT_STATE.f4 = false,
				F5 => INPUT_STATE.f5 = false,
				F6 => INPUT_STATE.f6 = false,
				F7 => INPUT_STATE.f7 = false,
				F8 => INPUT_STATE.f8 = false,
				F9 => INPUT_STATE.f9 = false,
				F10 => INPUT_STATE.f10 = false,
				F11 => INPUT_STATE.f11 = false,
				F12 => INPUT_STATE.f12 = false,
				PrintScreen => INPUT_STATE.print_screen = false,
				ScrollLock => INPUT_STATE.scroll_lock = false,
				Pause => INPUT_STATE.pause = false,
				Insert => INPUT_STATE.insert = false,
				Delete => INPUT_STATE.delete = false,
				Home => INPUT_STATE.home = false,
				End => INPUT_STATE.end = false,
				PgUp => INPUT_STATE.pg_up = false,
				PgDown => INPUT_STATE.pg_down = false,
				Menu => INPUT_STATE.menu = false,
				ArrowLeft => INPUT_STATE.arrow_left = false,
				ArrowUp => INPUT_STATE.arrow_up = false,
				ArrowRight => INPUT_STATE.arrow_right = false,
				ArrowDown => INPUT_STATE.arrow_down = false,
				Tilde => INPUT_STATE.tilde = false,
				Num1 => INPUT_STATE.num1 = false,
				Num2 => INPUT_STATE.num2 = false,
				Num3 => INPUT_STATE.num3 = false,
				Num4 => INPUT_STATE.num4 = false,
				Num5 => INPUT_STATE.num5 = false,
				Num6 => INPUT_STATE.num6 = false,
				Num7 => INPUT_STATE.num7 = false,
				Num8 => INPUT_STATE.num8 = false,
				Num9 => INPUT_STATE.num9 = false,
				Num0 => INPUT_STATE.num0 = false,
				Q => INPUT_STATE.q = false,
				W => INPUT_STATE.w = false,
				E => INPUT_STATE.e = false,
				R => INPUT_STATE.r = false,
				T => INPUT_STATE.t = false,
				Y => INPUT_STATE.y = false,
				U => INPUT_STATE.u = false,
				I => INPUT_STATE.i = false,
				O => INPUT_STATE.o = false,
				P => INPUT_STATE.p = false,
				A => INPUT_STATE.a = false,
				S => INPUT_STATE.s = false,
				D => INPUT_STATE.d = false,
				F => INPUT_STATE.f = false,
				G => INPUT_STATE.g = false,
				H => INPUT_STATE.h = false,
				J => INPUT_STATE.j = false,
				K => INPUT_STATE.k = false,
				L => INPUT_STATE.l = false,
				Z => INPUT_STATE.z = false,
				X => INPUT_STATE.x = false,
				C => INPUT_STATE.c = false,
				V => INPUT_STATE.v = false,
				B => INPUT_STATE.b = false,
				N => INPUT_STATE.n = false,
				M => INPUT_STATE.m = false,
				Space => INPUT_STATE.space = false,
				Minus => INPUT_STATE.minus = false,
				Equals => INPUT_STATE.equals = false,
				BracketLeft => INPUT_STATE.bracket_left = false,
				BracketRight => INPUT_STATE.bracket_right = false,
				Backslash => INPUT_STATE.backslash = false,
				Semicolon => INPUT_STATE.semicolon = false,
				Apostrophe => INPUT_STATE.apostrophe = false,
				Enter => INPUT_STATE.enter = false,
				Comma => INPUT_STATE.comma = false,
				Period => INPUT_STATE.period = false,
				Slash => INPUT_STATE.slash = false,
				NumPad0 => INPUT_STATE.numpad0 = false,
				NumPad1 => INPUT_STATE.numpad1 = false,
				NumPad2 => INPUT_STATE.numpad2 = false,
				NumPad3 => INPUT_STATE.numpad3 = false,
				NumPad4 => INPUT_STATE.numpad4 = false,
				NumPad5 => INPUT_STATE.numpad5 = false,
				NumPad6 => INPUT_STATE.numpad6 = false,
				NumPad7 => INPUT_STATE.numpad7 = false,
				NumPad8 => INPUT_STATE.numpad8 = false,
				NumPad9 => INPUT_STATE.numpad9 = false,
				NumLock => INPUT_STATE.numlock = false,
				NumPadDec => INPUT_STATE.numpad_dec = false,
				NumPadDiv => INPUT_STATE.numpad_div = false,
				NumPadMult => INPUT_STATE.numpad_mult = false,
				NumPadSub => INPUT_STATE.numpad_sub = false,
				NumPadAdd => INPUT_STATE.numpad_add = false,
				NumPadEq => INPUT_STATE.numpad_eq = false,
				NumPadEnter => INPUT_STATE.numpad_enter = false,
			}
		}
		false
	}
}
