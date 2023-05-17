extern crate middleware;
extern crate protocol;
extern crate tcp_service;

use std::{io, thread};
// use tcp_service::raw::Host;
use tcp_service::{client, handshake, host};

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
        // let broadcaster_thread = thread::spawn(|| {
        // let mut host = Host::new("localhost".to_string(), 8000, 1000);
        // broadcaster::init_handshake(&mut host);
        // });

        println!("Press Enter after all the clients are connected");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // let host_thread = thread::spawn(|| {
        //     // Wait for broadcaster to start
        //     thread::sleep(Duration::from_millis(100));
        //     host::stream_flush();
        // });

        // broadcaster_thread.join().unwrap();
        // host_thread.join().unwrap();
    } else if inp_num == 2 {
        client::start();
    } else {
        println!("Invalid input")
    }
}
