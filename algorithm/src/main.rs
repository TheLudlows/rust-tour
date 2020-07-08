#![feature(core)]

use std::cell::RefCell;

struct Foo;
impl Drop for Foo{
    fn drop(&mut self) {
       println!("dropped")
    }
}
fn main() {
    let f = Foo;
    if let Foo = f {
        println!("Foo");
    }
}