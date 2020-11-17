use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame, Command};
use mini_redis::cmd::Command::{Set, Get};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use bytes::Bytes;


type DB = Arc<Mutex<HashMap<String,Bytes>>>;
#[tokio::main]
async fn  main() {
    let mut listener = TcpListener::bind("localhost:8080").await.unwrap();
    let db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (socket,addr) = listener.accept().await.unwrap();
        let db = db.clone();
        println!("{}",addr);
        process(socket,db).await;
    }
}
async fn process(socket:TcpStream,db:DB) {
    let mut conn = Connection::new(socket);
    while let Some(frame) = conn.read_frame().await.unwrap() {
        let r = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                println!("{:?}",cmd);
                db.insert(cmd.key().to_string(),cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let mut db = db.lock().unwrap();
                println!("{:?}",cmd);
                if let Some(v) = db.get(cmd.key()) {
                    Frame::Bulk(v.clone())
                }else{
                    Frame::Null
                }
            }
            _ => {
                Frame::Simple("unimpl".to_string())
            }
        };
        conn.write_frame(&r).await.unwrap();
    }
}
