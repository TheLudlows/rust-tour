use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // 创建一个最大容量为32的通道
    let (mut tx, mut rx) = mpsc::channel(32);
    let mut tx2 = tx.clone();
    tokio::spawn(async move {
        tx.send("1111").await
    }
    );
    tokio::spawn(async move {
        tx2.send("2222").await
    }
    );
    while let Some(mess) = rx.recv().await {
        println!("{:?}", mess)
    }
    // ... 这里先休息一下
}