use std::{
    any::{type_name, type_name_of_val},
    collections::HashMap,
    mem::size_of_val,
    thread,
    time::Duration,
};

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
impl<F> Executor for F
where
    F: Fn(&str) -> Result<String, &'static str>,
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

    let c = |a: &str| println!("{}{}", a, s);
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
    let c = || {
        println!("data: {:?}", data);
        println!("v: {:?}, name: {:?}", v, name.clone());
    };
    let vec = vec!["1"];
    let c1 = move |greeting: String| (greeting, vec);

    c1("abc".to_string());
    //c1("abc".to_string());
    //println!("{}", type_name_of_val(&c))
    // 请问在这里，还能访问 name 么？为什么？
}
fn call_one(f: impl FnOnce()) {
    f()
}

struct Foo;

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
#[test]
fn fn_mut() {
    let mut name = String::from("hello");
    let mut name1 = String::from("hola");

    // 捕获 &mut name
    let mut c = || {
        name.push_str(" Tyr");
        println!("c: {}", name);
    };

    // 捕获 mut name1，注意 name1 需要声明成 mut
    let mut c1 = move || {
        name1.push_str("!");
        println!("c1: {}", name1);
    };

    c();
    c1();

    call_mut(&mut c);
    call_mut(&mut c1);

    call_once(c);
    call_once(c1);
}

// 在作为参数时，FnMut 也要显式地使用 mut，或者 &mut
fn call_mut(c: &mut impl FnMut()) {
    c();
}

// 想想看，为啥 call_once 不需要 mut？
fn call_once(c: impl FnOnce()) {
    c();
}
mod test_fn {
    #[test]
    fn fn_test() {
        let v = vec![0u8; 1024];
        let v1 = vec![0u8; 1023];

        // Fn，不移动所有权
        let mut c = |x: u64| v.len() as u64 * x;
        // Fn，移动所有权
        let mut c1 = move |x: u64| v1.len() as u64 * x;

        println!("direct call: {}", c(2));
        println!("direct call: {}", c1(2));

        println!("call: {}", call(3, &c));
        println!("call: {}", call(3, &c1));

        println!("call_mut: {}", call_mut(4, &mut c));
        println!("call_mut: {}", call_mut(4, &mut c1));

        println!("call_once: {}", call_once(5, c));
        println!("call_once: {}", call_once(5, c1));
    }

    fn call(arg: u64, c: &impl Fn(u64) -> u64) -> u64 {
        c(arg)
    }

    fn call_mut(arg: u64, c: &mut impl FnMut(u64) -> u64) -> u64 {
        c(arg)
    }

    fn call_once(arg: u64, c: impl FnOnce(u64) -> u64) -> u64 {
        c(arg)
    }
}
