use std::cell::RefCell;
use std::rc::{Weak, Rc};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
fn main() {
    let leaf = Rc::new(Node{
        value:10,
        parent:RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });
    let root = Rc::new(Node{
        value:1,
        parent:RefCell::new(Weak::new()),
        children: RefCell::new(vec![leaf.clone()])
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&root);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

}