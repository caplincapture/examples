use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
pub enum Event {
    Save,
    Load
}

pub type Subscriber = fn(file_path: String);

/// Publisher sends events to subscribers (listeners).
#[derive(Default)]
// notice the publisher as internal data structure and notification API
// works internally to the outward interfacing functions
// that said, 'events' are still callable from the exterior structure
// function simply returns internal storage
pub struct Publisher {
    pub events: HashMap<Event, Vec<Subscriber>>,
}

impl Publisher{
    pub fn subscribe(& mut self, event_type: Event, listener: Subscriber) {
        self.events.entry(event_type).or_default().push(listener);
    }
    pub fn unsubscribe(& mut self, event_type: Event, listener: Subscriber) {
        if let Some(subscribers) = self.events.get_mut(&event_type) {
            subscribers.retain(|&sub|sub != listener)
        }
    }
    pub fn notify(& mut self, event_type: Event, file_path: String) {
        if let Some(listeners) = self.events.get(&event_type) {
            for listener in listeners {
                listener(file_path.clone())
            }
        }
    }
}

