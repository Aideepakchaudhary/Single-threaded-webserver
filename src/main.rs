use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // It listen to the tcp socket server and bind it with the address.

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}