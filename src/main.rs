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
        .expect("Failed to get server ip");

    let inp_num = match input.trim().parse::<i32>() {
        Ok(x) => x,
        Err(_) => 0,
    };

    if inp_num == 1 {
        let mut server_ip = String::new();
        println!("Enter server ip: ");
        io::stdin()
            .read_line(&mut server_ip)
            .expect("Failed to get server ip");

        let host = raw::Host::new(server_ip.trim().to_string(), 8000, 1000);
        let mut client_latencies = raw::ClientLatencies::new();

        host::init(host, &mut client_latencies);
    } else if inp_num == 2 {
        let mut client_name = String::new();
        let mut client_ip = String::new();

        println!("Enter client name: ");
        io::stdin()
            .read_line(&mut client_name)
            .expect("Failed to get client name");

        println!("Enter server ip: ");
        io::stdin()
            .read_line(&mut client_ip)
            .expect("Failed to get server ip");

        client::handshake(
            client_name.trim().to_string(),
            client_ip.trim().to_string(),
            8000,
        );
    } else {
        println!("Invalid input")
    }
}
