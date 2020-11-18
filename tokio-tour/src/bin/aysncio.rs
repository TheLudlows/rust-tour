use std::io;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::io::SeekFrom::Start;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("readme.md").await?;
    let mut buffer = [0; 10];

    // 读取10个字节
    let n = f.read(&mut buffer[..]).await?;
    println!("The bytes: {:?}", &buffer[..n]);

    let mut v = Vec::new();
    f.seek(Start(0));
    f.read_to_end(&mut v).await?;
    println!("{:?}",v);
    Ok(())
}