use std::collections::{HashMap, VecDeque};

use crate::trains::Train;

// mediator has notification methods
// when action needing tobe mediated by a component gets taken
// it gets a reference to a mediator object
// updatable reference struct that isn't a refcell, rc, etc.
pub trait Mediator {
    fn notify_about_arrival(& mut self, train_name: &str) -> bool;
    fn notify_about_departure(& mut self, train_name: &str);
}

#[derive(Default)]
pub struct TrainStation {
    // takeaway here is that the mediator need not
    // be a substructure to be 'owned' by the train
    // station and passed around
    trains: HashMap<String, Box<dyn Train>>,
    train_queue: VecDeque<String>,
    train_on_platform: Option<String>,
}

impl Mediator for TrainStation {
    // train name is a reference to a string uniqueness
    fn notify_about_arrival(& mut self, train_name: &str) -> bool {
        if self.train_on_platform.is_some() {
            // should the name stay or go? rust problem par excellence
            // should the mediator touch the train station 'trains' object?
            self.train_queue.push_back(train_name.into());
            false
        } else {
            self.train_on_platform.replace(train_name.into());
            true
        }
    }

    fn notify_about_departure(& mut self, train_name: &str) {
        if Some(train_name.into()) == self.train_on_platform {
            self.train_on_platform = None;
        
            if let Some(next_train) = self.train_queue.pop_front() {

                self.train_on_platform = Some(next_train);
                self.train_queue.pop_front();
            }
        }
    }
}


impl TrainStation {
    pub fn accept(&mut self, train: impl Train + 'static) {
        train.arrive(self);
        self.trains.insert(train.name().clone(),Box::new(train));
    }
    
    pub fn depart(& mut self, name: &str) {
        let train = self.trains.remove(name);
        if let Some(train) = train {
            train.depart(self);
        } else {
            println!("'{}' is not in the station!", name);
        }
    }
}
