use std::mem;

struct A {
    a: usize,
    b: u32,
}

struct B {
    a: u32,
    b: usize,
}
#[repr(C)]
struct C {
    a: u8,
    b: u32,
    c: u16,
}

struct D {
    a: u8,
    b: u32,
    c: u16,
}

enum E {
    A(u32),
    B,
}

enum F {
    A(u64),
    B,
}

struct J<'a> {
    //a:i32,
    b: &'a [u8]
}


struct Foo {
    info: u32,
    data: [u8],
}

struct Empty {
    a: (),
    b: [u8; 0],
}

enum Void {}

struct Bar;

trait T {}

#[test]
fn test_mem() {
    println!("{}", mem::size_of::<A>());
    println!("{}", mem::size_of::<B>());

    println!("{}", mem::size_of::<C>());
    println!("{}", mem::size_of::<D>());
    println!("{}", mem::align_of::<C>());
    println!("{}", mem::align_of::<D>());

    println!("{}", mem::align_of::<A>());
    println!("{}", mem::size_of::<Option<i32>>());
    println!("{}", mem::size_of::<Option<u64>>());
    println!("E:{}", mem::size_of::<E>());
    println!("{}", mem::size_of::<F>());
    println!("{}", mem::align_of::<F>());
    println!("{}", mem::size_of::<&Foo>());
    println!("{}", std::any::type_name::<&Foo>());
    println!("{}", mem::size_of::<Bar>());
    println!("{}", mem::size_of::<Empty>());
    println!("{}", mem::size_of::<Void>());
    println!("{}", mem::size_of::<Box<dyn T>>());
    println!("{}", mem::size_of::<J>());
    println!("{}", mem::align_of::<J>());
    println!("{}", mem::align_of::<&i32>());

}