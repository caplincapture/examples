use crate::train_station::Mediator;

mod freight_train;
mod passenger_train;

pub use freight_train::FreightTrain;
pub use passenger_train::PassengerTrain;

pub trait Train {
    fn name(&self) -> &String;
    fn arrive(&self, mediator: &mut dyn Mediator);
    fn depart(&self, mediator: &mut dyn Mediator);
}