// Uncomment this block to pass the first stage
use std::net::TcpListener;
use std::io::Write;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running
    // tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("0.0.0.0:6379").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Accepted new connection");
                _ = stream.write(b"+PONG\r\n");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
