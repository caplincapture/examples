
use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;

struct BufferDecorator<'a> {
    data: &'a str
}

impl BufferDecorator<'_> {

    fn new(input: &str) -> Self {
        BufferDecorator { data: input }
    }
    
    fn read(&mut self, buffer: &mut [char]) {
        for byte in self.data.chars() {
            buffer[0] = byte
        }
    }
}


fn main() {
    let mut buf : [char; 10] = ['\0','\0','\0','\0','\0','\0','\0','\0','\0','\0'];

    // A buffered reader decorates a vector reader which wraps input data.
    let mut input = BufferDecorator::new("Input data");

    input.read(&mut buf).ok();

    print!("Read from a buffered reader: ");

    for byte in buf {
        print!("{}", char::from(byte));
    }

    println!();
}

