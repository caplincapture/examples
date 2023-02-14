#[derive(Default)]
pub struct Request {
    pub id: String,
    pub processone: bool,
    pub processtwo: bool,
    pub processthree: bool,
}

pub trait Process {

    fn execute(&mut self, request: &mut Request){
        self.handle(request);
    
        if let Some(handoff) = &mut self.next() {
            handoff.execute(request);
        }
    }
    fn handle(&mut self, request: &mut Request);
    fn next(&mut self) -> &mut Option<Box<dyn Process>>;
}

pub(self) fn into_next(
    process: impl Process + Sized + 'static,
) -> Option<Box<dyn Process>> {
    Some(Box::new(process))
}

#[derive(Default)]
pub struct LastProcess {
    next: Option<Box<dyn Process>>,
}

impl Process for LastProcess {
    fn handle(&mut self, request: &mut Request) {
        if request.processthree {
            println!("Last process done");
        } else {
            println!("Last process getting request info {}", request.id);
            request.processthree = true;
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
    fn handle(&mut self, request: &mut Request) {
        if request.processtwo {
            println!("The SecondProcess is already done");
        } else {
            println!("Second Process checking a Request {}", request.id);
            request.processtwo = true;
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
            println!("Process one handled the request");
        } else {
            println!("First process processed the request with id {}", request.id);
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
        id: "workload".into(),
        ..Request::default()
    };

    firstproc.execute(&mut request);

    println!("\nThe request has been already handled:\n");

    firstproc.execute(&mut request);
}