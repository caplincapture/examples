use std::collections::{HashMap, VecDeque};

use crate::trains::Train;

// Mediator has notification methods.
pub trait Mediator {
    fn notify_about_arrival(self) -> bool;
    fn notify_about_departure(self);
}

#[derive(Default)]
pub struct TrainStation {
    trains: HashMap<String, Box<dyn Train>>,
    train_queue: VecDeque<String>,
    train_on_platform: Option<String>,
}

impl TrainStation {
    pub fn accept(self, train: impl Train) {
        self.trains.insert(train.name().clone(), Box::new(train));
    }
    pub fn depart(self, name: &String) {
        self.trains.remove_entry(name);
    }
}

impl Mediator for TrainStation {
    fn notify_about_arrival(self, train_name: &str) -> bool {
        if self.trains.is_some() {
            self.train_queue.push_back(train_name.into());
            false
        } else {
            self.train_on_platform.replace(train_name.into());
            true
        }
    }

    fn notify_about_departure(self) {

    }
}