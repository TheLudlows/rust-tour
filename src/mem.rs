use std::mem;

struct A {
    a: usize,
    b: u32,
}

struct B {
    a: u32,
    b: usize,
}

struct C {
    a: u8,
    b: u32,
    c: u16,
}

#[test]
fn test_mem() {
    println!("{}", mem::size_of::<A>());
    println!("{}", mem::size_of::<B>());
    println!("{}", mem::size_of::<C>());
    println!("{}", mem::align_of::<C>());
}