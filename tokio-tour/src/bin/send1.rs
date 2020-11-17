use tokio::task::yield_now;
use std::rc::Rc;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        // 在 .await 之前作用域强制　rc drop了
        {
            let rc = Rc::new("hello");
            println!("{}", rc);
        }

        // rc 不再使用. 当任务返回到调度器后,  rc 不能再持续下去
        yield_now().await;
    });
}