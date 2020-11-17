use tokio::task::yield_now;
use std::rc::Rc;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        let rc = Rc::new("hello");

        // rs 在　.await后继续使用, 它必须持久化到 task　的　状态中才行
        //yield_now().await;

        println!("{}", rc);
    });
}