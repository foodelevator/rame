use crate::engine::EngineData;

pub trait Application {
    fn start(&mut self, ed: &mut EngineData);
    fn close(&mut self);
}