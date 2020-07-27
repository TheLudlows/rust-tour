use std::future::Future;
use futures::executor::block_on;

#[test]
fn main() {
    block_on(say());
    block_on(see());

}
fn say() -> impl Future {
    async {
        println!("bbb")
    }
}

async fn see() {
    println!("ccc")
}