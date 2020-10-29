#[test]
fn no_mutable_aliasing() {
    let mut name = String::from("Vivian");
    let nickname = &name[..];
    //let ref_mut = & mut name;
    println!("Hello there, {}!", nickname);

    let c = 'Q';

    // 赋值语句中左边的 `ref` 关键字等价于右边的 `&` 符号。
    let ref _c1 = c;
    let ref _c2 = &c;

}
