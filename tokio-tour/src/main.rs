mod executor;

use std::process::exit;
use std::net::TcpStream;
use tokio;

fn main() {
    let r = "127.0.0.1:8088".parse();
    if let Err(e) = r {
        println!("{}", e);
        exit(1)
    }
    let client = TcpStream::connect(&r.unwrap()).and_then(|s|
        io::write_all(s, "hello world\n")
            .then(|result| {
                println!("wrote to stream; success={:?}", result.is_ok());
                Ok(())
            })
    );
    tokio::run(client)
}
