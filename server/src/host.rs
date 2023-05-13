use std::{io::Write, net::TcpStream, thread, time::Duration};

pub fn start() {
    let mut stream = TcpStream::connect("127.0.0.1:8000").unwrap();
    println!("Sending packets");

    let mut packet_no = 0;

    loop {
        // send dummy message to the broadcaster
        stream
            .write_all(format!("{}\n", packet_no).as_bytes())
            .unwrap();

        // Pause for 100ms
        thread::sleep(Duration::from_millis(100));

        // increment packet count
        packet_no += 1;
    }
}
