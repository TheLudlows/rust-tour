
#![allow(unused_variables)]
fn main() {
    let data = "initial contents";
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    let s = String::new();

    let mut s = String::from("foo");
    let str = "bar";
    s.push_str(str);
    println!("{}",str);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    let s = "abcde".to_string();
    let c = s[0..2];

}