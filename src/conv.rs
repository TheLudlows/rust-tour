
struct Foo{
    a:i32
}

struct Bar{
    a:i32
}

#[test]
fn conv() {
    let f = Foo{a:10};
    // let b = f as Bar; can't work
    let t:&Bar = unsafe { &*(&f as *const Foo as *const Bar) };
}