use futures::executor;

async fn learn_song() {
    println!("Learn song!");
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
    executor::block_on(async_main());
    println!("Hello, world!");
}