

//#[derive(Debug)]
pub struct LoggingFacade<'a> {
    message: Loggable<'a>,
}

impl LoggingFacade<'_> {

    pub fn new() -> Self {
        let logged = Loggable::new();
        Self { 
            message: logged
        }
    }

    pub fn change_log(&mut self) -> Loggable {
        self.message.change()
    }
}

pub struct Loggable<'a> {
    msg: &'a str
}

impl Loggable<'_> {
    pub fn new() -> Self {
        Self {msg: "new"}
    }

    pub fn change(& mut self) -> Self {
        Self {msg: "changed"}
    } 
}

fn main() {
    let mut logger = LoggingFacade::new();
    let changed = logger.change_log();
    changed
}
