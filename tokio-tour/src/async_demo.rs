use futures::executor::block_on;
use failure::_core::future::Future;

async fn say() {
    println!("say hello")
}

async fn do_some() {
    println!("do some thing")
}

async fn all() {
    say().await;
    do_some().await
}

#[test]
fn start() {
    futures::executor::block_on(all())
}