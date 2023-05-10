// Uncomment this block to pass the first stage
use std::io::{Read, Write};
use std::net::TcpListener;

const MESSAGE_SIZE: usize = 128;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    let mut n = 1;
    for stream in listener.incoming() {
        n += 1;
        println!("{}", n);

        match stream {
            Ok(mut _stream) => {
                loop {
                    println!("Recieved message");
                    // Array with a fixed size
                    let mut rx_bytes = [0u8; MESSAGE_SIZE];
                    // Read from the current data in the TcpStream
                    let bytes_read = _stream.read(&mut rx_bytes).expect("read err");
                    println!("bytes read: {}", bytes_read);
                    if bytes_read == 0 {
                        break;
                    }
                    let received = std::str::from_utf8(&rx_bytes).expect("valid utf8");
                    println!("{}", received);

                    _stream.write(b"+PONG\r\n").expect("write err");
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
