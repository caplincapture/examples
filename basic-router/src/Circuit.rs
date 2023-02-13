

#[derive(Clone, Default)]
struct RouterPrototype {
}


impl RouterPrototype {
    fn new() -> RouterPrototype {
        RouterPrototype {}
    }
}

#[test]
fn main () {
    let router_prototype = RouterPrototype::new();
    let prototyped = router_prototype.clone();
    prototyped.clone();
}

