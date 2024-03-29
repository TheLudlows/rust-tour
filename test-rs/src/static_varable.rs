
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::str::Chars;
use std::sync::{Arc, Mutex};

static mut COUNTER: u64 = 0;
lazy_static! {
    static ref HASHMAP: Arc<Mutex<HashMap<u32, &'static str>>> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        Arc::new(Mutex::new(m))
    };
}

fn main() {
    let mut map = HASHMAP.lock().unwrap();
    map.insert(3, "waz");

    println!("map: {:?}", map);

}
