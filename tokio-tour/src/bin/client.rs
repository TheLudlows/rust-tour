use bytes::Bytes;
use mini_redis::client;
use mini_redis::Result;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

use crate::Command::{Get, Set};

type RespSender<T> = oneshot::Sender<T>;

#[tokio::main]
pub async fn main() -> Result<()> {
    let (mut job_sender, mut rx) = mpsc::channel(32);
    let mut job_sender2 = job_sender.clone();
    // move 关键字用来移动　rx 所有权到task中去
    let manager = tokio::spawn(async move {
        // 建立与Server的链接
        let mut client = client::connect("127.0.0.1:8080").await.unwrap();
        // 开始接收消息
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Get { key, resp_sender } => {
                    let resp = client.get(&key).await;
                    resp_sender.send(resp.unwrap_or(None)).unwrap();
                }
                Set { key, val, resp_sender } => {
                    let resp = client.set(&key, Bytes::from(val)).await;
                    resp_sender.send(resp.unwrap()).unwrap();
                }
            }
        }
    });

    // 产生两个任务一个得到key值,一个设置key的值
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "hello".to_string(),
            resp_sender: resp_tx,
        };
        job_sender.send(cmd).await.unwrap();
        let resp = resp_rx.await;
        println!("{:?}", resp);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp_sender: resp_tx,
        };
        job_sender2.send(cmd).await.unwrap();
        let resp = resp_rx.await;
        println!("{:?}", resp);
    });
    t1.await;
    t2.await;

    Ok(())
}

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp_sender: RespSender<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Vec<u8>,
        resp_sender: RespSender<()>,
    },
}
