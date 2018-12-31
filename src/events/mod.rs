mod event;
mod event_listener;
mod event_queue;
mod keyboard_events;
mod mouse_events;
mod window_events;

pub type EventBox = Box<dyn Event>;

pub use self::event::Event;
pub use self::event_listener::EventListener;
pub use self::event_queue::EventQueue;
pub use self::keyboard_events::*;
pub use self::mouse_events::*;
pub use self::window_events::*;
