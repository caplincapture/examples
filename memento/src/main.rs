
trait Memento<T> {
    fn restore(&self) -> Originator;
    fn print(&self);
}

pub struct Originator {
    state: u32
}

impl Originator{
    fn save(&self) -> OriginatorBackup {
        OriginatorBackup { state: self.state }
    }
}

pub struct OriginatorBackup {
    state: u32
}

impl Memento<Originator> for OriginatorBackup {
    fn restore(&self) -> Originator {
        Originator { state: self.state }
    }
    fn print(&self){
        println!("Originator backup: '{}'", self.state);
    }
}

fn main() {
    let mut history = Vec::<OriginatorBackup>::new();

    let mut originator = Originator { state: 0 };

    originator.state = 1;
    history.push(originator.save());

    originator.state = 2;
    history.push(originator.save());

    for moment in history.iter() {
        moment.print();
    }

    let originator = history.pop().unwrap().restore();
    println!("Restored to state: {}", originator.state);

    let originator = history.pop().unwrap().restore();
    println!("Restored to state: {}", originator.state);
}