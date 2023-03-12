

pub trait Strategy {
    fn route(&self, source: &str, dest: &str);
}

struct WalkingStrategy;
struct PublicTransportStrategy;

impl Strategy for WalkingStrategy {
    fn route(&self, source: &str, dest: &str) {
        println!(
            "Walking route from {} to {}: 3 km, 5 min",
            source, dest
        );
    }
}

impl Strategy for PublicTransportStrategy {
    fn route(&self, source: &str, dest: &str) {
        println!(
            "Public Transport route from {} to {}: 3 km, 5 min",
            source, dest
        );
    }
}

pub struct Navigator<T> {
    strategy: T
}

impl <T: Strategy> Navigator<T> {
    fn new(strategy: T) -> Self {
        Self {
            strategy
        }
    }
    fn route(&self, source: &str, dest: &str) {
        self.strategy.route(source, dest)
    }
}

fn main() {
    let navigator = Navigator::new(WalkingStrategy);
    navigator.route("Home", "Club");
    navigator.route("Club", "Work");

    let navigator = Navigator::new(PublicTransportStrategy);
    navigator.route("Home", "Club");
    navigator.route("Club", "Work");
}