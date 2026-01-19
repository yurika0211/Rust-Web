use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3001").unwrap();
    println!("Running on port 3001……");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Got a connection!");
        let mut buffer = [1; 1024];

        stream.read(&mut buffer).unwrap();

        stream.write(&mut buffer).unwrap();

    }
}
