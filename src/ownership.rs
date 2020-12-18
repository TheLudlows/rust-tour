fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);
    clone_test();
    test_fn();
    test_move_fn();
    ref_test();
    change_test();
    limit();
}

fn clone_test() {
    let s1 = String::from("four");
    let s2 = s1.clone();
    println!("{}{}", s1, s2);
}

fn test_fn() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);  // s 的值移动到函数里
    // s 到这里不再有效

    let x = 5;                 // x 进入作用域

    makes_copy(x);      // x 应该移动函数里，
    // 但 i32 是 Copy 的，所以在后面可继续使用 x

    // 这里x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
    // 所以不会有特殊操作
}

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作


fn test_move_fn() {
    let s1 = gives_ownership();                 // gives_ownership 将返回值移给 s1

    let s2 = String::from("hello");             // s2 进入作用域

    let s3 = takes_and_gives_back(s2);  // s2 被移动到 takes_and_gives_back 中,
    // 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，所以什么也不会发生
//s1 移出作用域并被丢弃

fn gives_ownership() -> String {             // gives_ownership 将返回值移动给调用它的函数

    let some_string = String::from("hello"); // some_string 进入作用域.

    some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

    a_string                                          // 返回 a_string 并移出给调用的函数
}

fn ref_test() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_test() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("change to {}", s)
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn limit() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!(" {}", r2);
}

fn limit2() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &mut s;

    println!(" {}", r2);
}

fn dangling() {
    let s = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}

#[test]
fn test() {
    let mut foo = Foo;
    let mut loan = foo.mutate_and_share();
    foo.share();
    loan.share();
}


struct Foo;

impl Foo {
    fn mutate_and_share(&mut self) -> &Self { &*self }
    fn share(&self) {}
}

