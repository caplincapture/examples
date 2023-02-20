mod advanced;
mod basic;

pub use advanced::AdvancedRemove;
pub use basic::BasicRemote;

pub trait Remote<D> {
    fn power(&mut self);
}

pub trait HasMutableDevice<D> {
    fn device(& mut self)-> &mut D;
}