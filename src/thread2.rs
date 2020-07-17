use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let vv = & v;
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}