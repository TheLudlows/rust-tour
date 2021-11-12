#![allow(unused_variables)]

#[test]
fn main() {
    let data = "initial contents";
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    let s = String::new();

    let mut s = String::from("foo");
    let str = "bar";
    s.push_str(str);
    println!("{}", str);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    let s = "世界".to_string();
    // let c = &s[0..1];编译不会错误，执行会报错

    let zh = "你好啊";
    let en = "hello !";

    println!("zh size {}", zh.len());

    for i in zh.chars() {
        println!("{}", i)
    }

    for i in en.bytes() {
        println!("{}", i)
    }
}
use std::fmt;

#[test]
fn main1() {
    let s = String::from("hello");
    // &String 会被解引用成 &str
    print_slice(&s);
    // &s[..] 和 s.as_str() 一样，都会得到 &str
    print_slice(&s[..]);

    // String 支持 AsRef<str>
    print_slice1(&s);
    print_slice1(&s[..]);
    print_slice1(s.clone());

    // String 也实现了 AsRef<[u8]>，所以下面的代码成立
    // 打印出来是 [104, 101, 108, 108, 111]
    print_slice2::<_,str>(&s);
    print_slice2::<_,str>(&s[..]);
    print_slice2::<_,str>(s);
}

fn print_slice(s: &str) {
    println!("{:?}", s);
}

fn print_slice1<T: AsRef<str>>(s: T) {
    println!("{:?}", s.as_ref());
}

fn print_slice2<T, U>(s: T)
    where
        T: AsRef<U>,
        U: fmt::Debug + ?Sized,
{
    println!("{:?}", s.as_ref());
}