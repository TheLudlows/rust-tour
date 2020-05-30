use std::thread;
use std::time::Duration;

fn main() {
    let h = thread::spawn(|| {
        println!("thread running");
        thread::sleep(Duration::from_secs(1));
        println!("thread run end");

    });
    h.join();
    println!("main thread");
}