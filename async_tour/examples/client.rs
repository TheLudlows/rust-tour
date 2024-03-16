use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 连接到服务器
    let mut socket = TcpStream::connect("127.0.0.1:8080").await?;

    // 准备要发送的数据
    let message = "Hello, server!";
    let message_bytes = message.as_bytes();

    // 发送数据
    socket.write_all(message_bytes).await?;

    // 可以选择读取服务器的响应，如下所示
    let mut buf = [0u8; 1024];
    let amount = socket.read(&mut buf).await?;
    let response = String::from_utf8_lossy(&buf[..amount]);
    println!("Received response: {}", response);

    // 关闭socket连接
    socket.shutdown().await?;

    Ok(())
}