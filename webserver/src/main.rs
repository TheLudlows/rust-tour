use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    let pool:ThreadPool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        //handle_connection(stream);
       /* thread::spawn(|| {
            handle_connection(stream);
        });*/
        pool.execute(||{
            handle_connection(stream);
        });
    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer);
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes());
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));


}
