use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[derive(Serialize, Deserialize, Debug)]
struct Packet {
    content: String,
    timestamp: i64,
}

#[tokio::main]
pub async fn start() {
    const PORT: i32 = 8000;

    println!("Starting as a host at port {}", PORT);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT))
        .await
        .unwrap();

    let (tx, _rx) = broadcast::channel(100);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (read, mut write) = socket.split();

            let mut reader = BufReader::new(read);

            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line)=>{
                        if result.unwrap()==0{
                            break;
                        }

                        let now = Utc::now();

                        // to remove \n from string
                        trim_newline(&mut line);

                        let message = Packet {
                            content:line.clone(),
                            timestamp: now.timestamp_millis()
                        };

                        tx.send((format!("{}\n",serde_json::to_string(&message).unwrap()), addr)).unwrap();
                        line.clear();
                    }

                    result = rx.recv()=>{
                        let (msg, other_addr) = result.unwrap();

                        if addr !=other_addr{
                        write.write_all(msg.as_bytes()).await.unwrap();
                    }
                    }
                }
            }
        });
    }
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
