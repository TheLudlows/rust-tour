use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();
    heap.push(5);
    heap.push(10);
    heap.push(20);
    heap.push(0);
    println!("{}",heap.pop().unwrap())
}