use crate::World;

pub trait System {
    fn init(&mut self, _world: &mut World) { }
    fn run(&mut self, world: &mut World);
}


