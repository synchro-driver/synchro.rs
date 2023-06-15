extern crate middleware;
extern crate protocol;
extern crate tcp_service;

use std::io;
use tcp_service::{client, host, raw};

fn main() {
    println!("Synchro Studio 🎵");

    println!("To start a host enter 1");
    println!("To connect to a host enter 2");
    println!("Enter selection :");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let inp_num = match input.trim().parse::<i32>() {
        Ok(x) => x,
        Err(_) => 0,
    };

    if inp_num == 1 {
        let host = raw::Host::new("127.0.0.1".to_string(), 8000, 1000);
        let mut client_latencies = raw::ClientLatencies::new();

        host::init(host, &mut client_latencies);
    } else if inp_num == 2 {
        client::handshake();
    } else {
        println!("Invalid input")
    }
}
