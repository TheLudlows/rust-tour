use std::ops::Add;

#[test]
fn main() {
    let mut a = Some(30);
    let mut i = (a.as_mut().unwrap());
    *i += 10;
    println!("{:?}", a)
}