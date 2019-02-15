use crate::engine::Engine;

pub trait Application {
    fn start<A>(&mut self, engine: &mut Engine<A>) where A: Application;
    fn close<A>(&mut self, engine: &mut Engine<A>) where A: Application;
}