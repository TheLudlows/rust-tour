use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer);
    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
        let response = "HTTP/1.1 200 OK\r\n\r\n aaa";
        stream.write(response.as_bytes());
    }
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));


}
