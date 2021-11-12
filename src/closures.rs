use std::time::Duration;

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

// 看看我给的 tonic 的例子，想想怎么实现让 27 行可以正常执行

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