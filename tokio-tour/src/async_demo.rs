async fn learn_sing() {
    println!("learn sing")
}

async fn sing_song() {
    println!("sing song")
}

async fn dance() {
    println!("dance")
}

async fn sing() {
    learn_sing().await;
    sing_song().await
}

async fn all() {
    futures::join!(sing(), dance());
}

#[test]
fn start() {
    futures::executor::block_on(all())
}