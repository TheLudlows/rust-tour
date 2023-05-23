use std::{time::Duration, thread, collections::HashMap, mem::size_of_val, any::{type_name, type_name_of_val}};

#[test]
fn closure_test() {
    let double = |num| num * 2;
    println!("{}", double(10));
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}

fn returns_closure0() -> Box<dyn Fn(i32) -> i32> {
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
#[test]
fn tedt_closure() {
    let a = 10;
    let f = || println!("{}", a);
    f();
}
pub trait Executor {
    fn execute(&self, cmd: &str) -> Result<String, &'static str>;
}

struct BashExecutor {
    env: String,
}

impl Executor for BashExecutor {
    fn execute(&self, cmd: &str) -> Result<String, &'static str> {
        Ok(format!(
            "fake bash execute: env: {}, cmd: {}",
            self.env, cmd
        ))
    }
}


fn main() {
    let env = "PATH=/usr/bin".to_string();

    let cmd = "cat /etc/passwd";
    let r1 = execute(cmd, BashExecutor { env: env.clone() });
    println!("{:?}", r1);

    let r2 = execute(cmd, |cmd: &str| {
        Ok(format!("fake fish execute: env: {}, cmd: {}", env, cmd))
    });
    println!("{:?}", r2);
}

fn execute(cmd: &str, exec: impl Executor) -> Result<String, &'static str> {
    exec.execute(cmd)
}
impl<F> Executor for F where F:Fn(&str) -> Result<String, &'static str>
{
    fn execute(&self, cmd: &str) -> Result<String, &'static str> {
       self(cmd)
    }
}
#[test]
fn test_cls() {
    let s = String::from("hello world");

    let handle = thread::spawn(move || {
        println!("moved: {:?}", s);
    });
    let s = String::from("abc");

    let c = |a:&str| println!("{}{}", a, s);
    handle.join().unwrap();
}
#[test]
fn test_len() {
        // 长度为 0
        let c1 = || println!("hello world!");
        // 和参数无关，长度也为 0
        let c2 = |i: i32| println!("hello: {}", i);
        let name = String::from("tyr");
        let name1 = name.clone();
        let mut table = HashMap::new();
        table.insert("hello", "world");
        // 如果捕获一个引用，长度为 8
        let c3 = || println!("hello: {}", name);
        // 捕获移动的数据 name1(长度 24) + table(长度 48)，closure 长度 72
        let c4 = move || println!("hello: {}, {:?}", name1, table);
        let name2 = name.clone();
        // 和局部变量无关，捕获了一个 String name2，closure 长度 24
        let c5 = move || {
            let x = 1;
            let name3 = String::from("lindsey");
            println!("hello: {}, {:?}, {:?}", x, name2, name3);
        };
    
            println!(
            "c1: {}, c2: {}, c3: {}, c4: {}, c5: {}, main: {}",
            size_of_val(&c1),
            size_of_val(&c2),
            size_of_val(&c3),
            size_of_val(&c4),
            size_of_val(&c5),
            size_of_val(&main),
        )
    
}
#[test]
fn test_c() {
    let name = String::from("Tyr");
    let vec = vec!["Rust", "Elixir", "Javascript"];
    let v = &vec[..];
    let data = (1, 2, 3, 4);
    let c = move || {
        println!("data: {:?}", data);
        println!("v: {:?}, name: {:?}", v, name.clone());
    };
    c();
    call_one(c);
    //println!("{}", type_name_of_val(&c))
    // 请问在这里，还能访问 name 么？为什么？
}
fn call_one(f: impl FnOnce()) {
    f()
}


struct  Foo;

impl Foo {
    fn bar(self) {
        println!("11");
    }
}

#[test]
fn test_self() {
    let foo = Foo;
    foo.bar();
    //foo.bar();

}
