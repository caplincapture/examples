use super::Train;
use crate::train_station::Mediator;

pub struct PassengerTrain {
    name: String,
}

impl PassengerTrain {
    pub fn new(self) -> Self {
        Self {name: ""}
    }
    pub fn new(self) -> Self {
        Self {name: ""}
    }
}

impl Train for PassengerTrain {
    fn name(name: &String) -> &String {
        name
    }

    fn arrive(self, mediator: &impl Mediator) {
        if !mediator.notify_about_arrival() {
            println!("Passenger train {}: Arrival blocked, waiting", self.name);
            return;
        }
        println!("Passenger train {}: Arrived", self.name);
    }

    fn depart(self, mediator: &impl Mediator) {
        println!("Passenger train {}: Leaving", self.name);
        mediator.notify_about_departure();
    }
}