use std::collections::{VecDeque, LinkedList};

fn main() {
    let mut dq = VecDeque::new();
    dq.push_front("a");
    assert_eq!(dq.get(0),Some(&"a"));
    dq.push_back("b");
    assert_eq!(dq.get(0),Some(&"a"));

    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    for v in list.iter(){
        println!("{}",v)
    }
}