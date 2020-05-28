enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;

fn main() {
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    let b = List::Cons(3, a.clone());
    let c = List::Cons(4, a.clone());
}