extern crate rand;

use std::io::Write;
use std::net::TcpListener;
use std::thread;

// use rand::Rng;

fn main() {
    let acceptor = TcpListener::bind("127.0.0.1:2333").unwrap();
    println!("listening started, ready to accept");
    for stream in acceptor.incoming() {
        thread::spawn(|| {
            let mut stream = stream.unwrap();
            stream.write(b"Hello Rust\r\n")
                .unwrap();
        });
    }
}
