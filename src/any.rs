use std::any::{Any, TypeId};

#[test]
fn test() {
    struct Foo {
        x: u8,
        y: u8,
    }
    let f = Foo { x: 0, y: 10 };
    let str = "abc";
    let num = 200;
    let pointer = &num;
    println!("{:?}", (&f).type_id());
    println!("{:?}", f.type_id());

    println!("{:?}", (&dyn Any) f.type_id());
}