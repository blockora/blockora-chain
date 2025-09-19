use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

pub fn start_server(address: &str) {
    let listener = TcpListener::bind(address).expect("Failed to bind address");
    println!("🛰 Node listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("🔗 New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => eprintln!("❌ Connection failed: {}", e),
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                let text = String::from_utf8_lossy(&buffer[..n]).to_string();
                println!("📩 Received: {}", text);
                let _ = stream.write(b"ACK");
            }
            Err(_) => break,
        }
    }
}
