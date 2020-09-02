
#[test]
fn closure_test() {
    let double = |num| num * 2;
    println!("{}", double(10));
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}


fn returns_closure0() -> Box<Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_closure1() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_closure2() -> Box<impl Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_closure3() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

