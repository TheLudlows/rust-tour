use std::future::Future;
use std::thread::sleep;
use std::time::Duration;

use futures::{self, executor};

#[test]
fn main() {
    executor::block_on(say());
    executor::block_on(see());

    //say();
    //see();
}

fn say() -> impl Future {
    async {
        println!("bbb")
    }
}

async fn see() {
    println!("ccc")
}

async fn lean() {
    println!("lean")
}

async fn exe() {
    sleep(Duration::from_secs(5));
    println!("exe")
}

async fn exam() {
    println!("exam")
}

async fn ready() {
    lean().await;
    exe().await;
}

async fn all() {
    futures::join!(ready(),exam());
}

#[test]
fn test_await() {
    executor::block_on(all())
}
