#[test]
fn no_mutable_aliasing() {
    let mut name = String::from("Vivian");
    let nickname = &name[..];
    //let ref_mut = & mut name;
    println!("Hello there, {}!", nickname);
}
