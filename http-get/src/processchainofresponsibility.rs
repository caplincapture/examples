#[derive(Default)]
pub struct Request {
    pub id: String,
    pub processone: bool,
    pub processtwo: bool,
    pub processthree: bool,
}

pub trait Process {

    fn execute(&mut self, request: Request){
        self.handle(request);
    
        if let Some(handoff) = &mut self.next() {
            handoff.execute(request);
        }

    fn handle(&mut self, request: &mut Request);
    fn next(&mut self) -> &mut Option<Box<dyn Process>>;
    }
}

pub(self) fn into_next(
    process: impl Process + Sized + 'static,
) -> Option<Box<dyn Process>> {
    Some(Box::new(process))
}

pub struct LastProcess {
    next: Option<Box<dyn Process>>,
}

impl Process for LastProcess {
    fn handle(&mut self, Request: &mut Request) {
        if Request.processthree {
            println!("Payment done");
        } else {
            println!("LastProcess getting money from a Request {}", Request.id);
            Request.processthree = true;
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn Process>> {
        &mut self.next
    }
}

pub struct SecondProcess {
    next: Option<Box<dyn Process>>,
}

impl SecondProcess {
    pub fn new(next: impl Process + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
}

impl Process for SecondProcess {
    fn handle(&mut self, Request: &mut Request) {
        if Request.processtwo {
            println!("A SecondProcess checkup is already done");
        } else {
            println!("SecondProcess checking a Request {}", Request.id);
            Request.processtwo = true;
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn Process>> {
        &mut self.next
    }
}

pub struct FirstProcess {
    next: Option<Box<dyn Process>>,
}

impl FirstProcess {
    pub fn new(next: impl Process + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
}

impl Process for FirstProcess {
    fn handle(&mut self, request: &mut Request) {
        if request.processone {
            println!("Medicine is already given to a Request");
        } else {
            println!("FirstProcess giving medicine to a Request {}", request.id);
            request.processone = true;
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn Process>> {
        &mut self.next
    }
}

fn main() {
    let thirdproc = LastProcess::default();
    let secondproc = SecondProcess::new(thirdproc);
    let mut firstproc = FirstProcess::new(secondproc);

    let mut request = Request {
        id: "John".into(),
        ..Request::default()
    };

    // Reception handles a patient passing him to the next link in the chain.
    // Reception -> Doctor -> Medical -> Cashier.
    firstproc.execute(&mut request);

    println!("\nThe patient has been already handled:\n");

    firstproc.execute(&mut request);
}