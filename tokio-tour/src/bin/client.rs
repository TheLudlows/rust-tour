use mini_redis::client;
use mini_redis::Result;

#[tokio::main]
pub async fn main() -> Result<()> {
    // 打开链接到mini-redis的链接
    let mut client = client::connect("127.0.0.1:8080").await?;
    client.set("hello", "world".into()).await?;
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);
    Ok(())
}