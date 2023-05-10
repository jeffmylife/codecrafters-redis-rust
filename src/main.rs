// Uncomment this block to pass the first stage
use std::io::{Read, Write};
use std::net::TcpListener;

const MESSAGE_SIZE: usize = 128;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                // Array with a fixed size
                let mut rx_bytes = [0u8; MESSAGE_SIZE];
                // Read from the current data in the TcpStream
                _stream.read(&mut rx_bytes).expect("read err");
                // Write PONG no matter what
                _stream.write(b"+PONG\r\n").expect("write err");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
