use std::{future::join, sync::{ Arc}, time::Duration, thread, };
use anyhow::Result;
use blake3::Hasher;
use chrono::Local;
use tokio::{try_join, net::TcpListener, sync::{Mutex, oneshot, mpsc}};
use tokio_util::codec::{Framed, LinesCodec};
use toml::Value;
use rayon::prelude::*;

#[tokio::test]
async fn test_async() -> Result<()> {
    let start = Local::now().timestamp();
    let f1 = tokio::fs::read_to_string("../Cargo.toml");
    let f2 = tokio::fs::read_to_string("../cargo.lock");

    let (c1, c2) = try_join!(f1,f2)?;
    let yaml1 = toml2yaml(&c1)?;
    let yaml2 = toml2yaml(&c2)?;
    let f3 = tokio::fs::write("1.ymal", yaml1);
    let f4 = tokio::fs::write("2.ymal", yaml2);
    try_join!(f3,f4)?;
    let end = Local::now().timestamp();
    println!("{}", end - start);
    Ok(())
}
fn toml2yaml(content: &str) -> Result<String> {
    let value: Value = toml::from_str(&content)?;
    Ok(serde_yaml::to_string(&value)?)
}


#[tokio::test]
async fn test_mutex() -> Result<()> {
    let db1 = Arc::new(Mutex::new(1));
    let db2 = Arc::clone(&db1);

    tokio::spawn(async move {
        let mut db = db1.lock().await;
        // 因为拿到的 MutexGuard 要跨越 await，所以不能用 std::sync::Mutex
        // 只能用 tokio::sync::Mutex
        //let affected = db.commit().await?;
        println!("db1: Total affected rows: {}", db);
        Ok::<_, anyhow::Error>(())
    });

    tokio::spawn(async move {
        let mut db = db2.lock().await;
        //let affected = db.commit().await?;
        println!("db2: Total affected rows: {}", db);

        Ok::<_, anyhow::Error>(())
    });

    // 让两个 task 有机会执行完
    tokio::time::sleep(Duration::from_millis(1)).await;

    Ok(())
}


pub const PREFIX_ZERO: &[u8] = &[0, 0, 0];

#[tokio::test]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("listen to: {}", addr);

    // 创建 tokio task 和 thread 之间的 channel
    let (sender, mut receiver) = mpsc::unbounded_channel::<(String, oneshot::Sender<String>)>();

    // 使用 thread 处理计算密集型任务
    thread::spawn(move || {
        // 读取从 tokio task 过来的 msg，注意这里用的是 blocking_recv，而非 await
        while let Some((line, reply)) = receiver.blocking_recv() {
            // 计算 pow
            let result = match pow(&line) {
                Some((hash, nonce)) => format!("hash: {}, once: {}", hash, nonce),
                None => "Not found".to_string(),
            };
            // 把计算结果从 oneshot channel 里发回
            if let Err(e) = reply.send(result) {
                println!("Failed to send: {}", e);
            }
        }
    });

    // 使用 tokio task 处理 IO 密集型任务
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Accepted: {:?}", addr);
        let sender1 = sender.clone();
        tokio::spawn(async move {
            // 使用 LinesCodec 把 TCP 数据切成一行行字符串处理
            let framed = Framed::new(stream, LinesCodec::new());
            // split 成 writer 和 reader
            let (mut w, mut r) = framed.split();
            while let Some(Ok(line)) = r.next().await {
                // 为每个消息创建一个 oneshot channel，用于发送回复
                let (reply, reply_receiver) = oneshot::channel();
                sender1.send((line, reply))?;

                // 接收 pow 计算完成后的 hash 和 nonce
                if let Ok(v) = reply_receiver.await {
                    w.send(format!("Pow calculated: {}", v)).await?;
                }
            }
            Ok::<_, anyhow::Error>(())
        });
    }
}

// 使用 rayon 并发计算 u32 空间下所有 nonce，直到找到有头 N 个 0 的哈希
pub fn pow(s: &str) -> Option<(String, u32)> {
    let hasher = blake3_base_hash(s.as_bytes());
    let nonce = (0..u32::MAX).into_par_iter().find_any(|n| {
        let hash = blake3_hash(hasher.clone(), n).as_bytes().to_vec();
        &hash[..PREFIX_ZERO.len()] == PREFIX_ZERO
    });
    nonce.map(|n| {
        let hash = blake3_hash(hasher, &n).to_hex().to_string();
        (hash, n)
    })
}

// 计算携带 nonce 后的哈希
fn blake3_hash(mut hasher: blake3::Hasher, nonce: &u32) -> blake3::Hash {
    hasher.update(&nonce.to_be_bytes()[..]);
    hasher.finalize()
}
use tokio::fs;


#[tokio::test]
async fn test_async() {
    let r = fs::read_to_string("./Cargo.yaml").await;
    println!("{:?}", r);
    
}



// 计算数据的哈希
fn blake3_base_hash(data: &[u8]) -> Hasher {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher
}