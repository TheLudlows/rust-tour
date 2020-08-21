use futures::executor;
use futures::join;
use async_std::task::sleep;
use std::time::Duration;

async fn learn_song() {
    println!("Learn song!");
    let sleep = sleep(Duration::from_secs(1));
    join!(sleep);
}

async fn sing_song() {
    println!("Sing song!");
}

async fn dance() {
    println!("Dance!");
}

async fn learn_and_sing_song() {
    learn_song().await;
    sing_song().await;
}

async fn async_main() {
    let f1 = learn_and_sing_song();
    let f2 = dance();
    f1.await;
    f2.await;
    
}

#[test]
fn test() {
    executor::block_on(learn_and_sing_song());
}