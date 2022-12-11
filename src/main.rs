use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Write;
use std::io::Read;


/// helper function to handle a connection from a client
fn handle_connection(mut stream: TcpStream) {
    println!("Accepted new connection");
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf).unwrap();
        println!("{bytes_read:?}");
        if bytes_read == 0 {
            println!("Client closed the connection");
            break;
        }
        let _ = stream.write("+PONG\r\n".as_bytes());
    }

}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running
    // tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("0.0.0.0:6379").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream)
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
