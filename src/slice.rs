
#![allow(unused_variables)]
fn main() {
    let mut s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let head = &s[..5];
    let tail = &s[6..];

    let all = &s[0..11];
    let all_ = &s[..];

    //s.clear();
    println!("{}", all_)

}
