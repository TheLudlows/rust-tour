use std::collections::HashMap;
use std::thread;
use std::time::Instant;

const NUM_ELEMENTS: usize = 1000000;

type HeavyThings = HashMap<usize, Vec<usize>>;

#[test]
fn main() {
    let heavy_things_1 = make_heavy_things();
    let heavy_things_2 = make_heavy_things();
    let len = log_time("drop in another thread", || {
        fn_that_drops_heavy_things_in_another_thread(heavy_things_2)
    });
    assert_eq!(len, NUM_ELEMENTS);
    let len = log_time("drop in this thread", || {
        fn_that_drops_heavy_things(heavy_things_1)
    });
    assert_eq!(len, NUM_ELEMENTS);
}

fn make_heavy_things() -> HeavyThings {
    (1..=NUM_ELEMENTS).map(|v| (v, vec![v])).collect()
}

fn fn_that_drops_heavy_things(things: HeavyThings) -> usize {
    things.len()
}

fn fn_that_drops_heavy_things_in_another_thread(things: HeavyThings) -> usize {
    let len = things.len();
    thread::spawn(move || drop(things)).join().unwrap();
    len
}

fn log_time<T, F: FnOnce() -> T>(name: &str, f: F) -> T {
    let time = Instant::now();
    let result = f();
    println!("{} {:?}", name, time.elapsed());
    result
}