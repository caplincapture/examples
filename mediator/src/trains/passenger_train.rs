use crate::train_station::Mediator;

use super::Train;

pub struct PassengerTrain {
    name: String
}

impl PassengerTrain {
    pub fn new(name: &'static str) -> Self {
        Self { name: name.into() }
    }
}

impl Train for PassengerTrain {
    fn name(&self) -> &String{
        &self.name
    }
    fn arrive(&self, mediator: &mut dyn Mediator) {
        if !mediator.notify_about_arrival(&self.name) {
            println!("Freight train {}: Arrival blocked, waiting", self.name());
            return;
        }
        println!("Freight train {}: Arrived", self.name());
    }

    fn depart(&self, mediator: &mut dyn Mediator) {
        println!("Freight train {}: Leaving", self.name());
        mediator.notify_about_departure(self.name());
    }
}

