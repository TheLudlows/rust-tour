use std::collections::{HashMap, BTreeMap};

fn main() {
    let mut hashmap = HashMap::new();
    let mut treemap = BTreeMap::new();
    hashmap.insert('a',3);
    hashmap.insert('b',1);
    hashmap.insert('c',2);
    treemap.insert('a',3);
    treemap.insert('b',1);
    treemap.insert('c',2);
    println!("{:?}",hashmap);
    println!("{:?}",treemap);
}