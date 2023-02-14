
#![allow(elided_lifetimes_in_paths)]

use std::{fmt, thread::spawn, io};

struct Err {}

#[derive(Debug)]
struct SocketAddr {}

#[derive(Clone)]
struct TcpStream {}

impl fmt::Display for TcpSocketServer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.address)
    }
}

struct TcpSocketServer {
    address: String
}

impl TcpSocketServer {
    fn new() -> Self {
        TcpSocketServer { address: "".to_string() }
    }
    fn bind(self, addr: &str) -> Self {
        TcpSocketServer { address: addr.to_string() }
    }
    fn accept(&self) -> Result<(TcpStream, SocketAddr), ()> {
        let stream = TcpStream {};
        let socketAddress = SocketAddr {};
        Ok((stream, socketAddress))
    } 
}

pub trait TryClone : Sized {
    type Error;
    fn try_clone(&self) -> Result<Self, Self::Error>;
}


impl <T: Clone> TryClone for T {
    type Error = Err;
    fn try_clone(&self) -> Result<Self, Self::Error> {
        Ok(self.clone())
    }
}

/// Accept connections forever, spawning a thread for each one.
fn echo_main(addr: &str) -> io::Result<()> {
    let tcpserver = TcpSocketServer::new();
    let listener = tcpserver.bind(addr);
    println!("listening on {}", addr);
    loop {
        // Wait for a client to connect.
        let (mut stream, addr) = listener.accept()?;
        println!("connection received from {:?}", addr);
        //copy and try unimplemented
       /*  // Spawn a thread to handle this client.
        let mut write_stream = stream.try_clone()?;
        spawn(move || {
            // Echo everything we receive from `stream` back to it.
            copy(&mut stream, &mut write_stream)
                .expect("error in client thread: ");
            println!("connection closed");
        }); */
    }
}

fn main() {
    echo_main("127.0.0.1:17007").expect("error: ");
}
