use futures::{StreamExt};
use async_tour::{Executor, MyTcpListener};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

fn main() {

    let ex = Executor::new();
    ex.block_on(server);

}
async fn  server() {

    let mut l = MyTcpListener::bind("127.0.0.1:8080").unwrap();
    while let Some(ret) = l.next().await {
        if let Ok((mut stream, addr)) = ret {
            println!("accept conn from {}", addr);
            Executor::spawn(async move{
                let mut buf =[0;1024];
                loop {
                   match stream.read(&mut buf).await {
                       Ok(r) => {
                           stream.write_all(&buf[0..r]).await.unwrap();
                       }
                       Err(_) => {}
                   }
                }
            });
        }
    }

}