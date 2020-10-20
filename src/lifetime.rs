use std::fmt::Debug;

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

fn print_ref<'a, T>(t: &'a T)
where
    T: 'a + Debug,
{
    println!("{:?}", t)
}

fn print<T>(t: T)
where
    T: Debug,
{
    println!("{:?}", t)
}

fn main() {
    let r = Ref(&1);
    print_ref(&r);
    print(r);
    let a: &str = "a";
    let b = a.to_string();
}

#[test]
fn lt_test() {
    let mut data = vec![1, 2, 3];
    let x = &data[0];
    //data.push(4);
    println!("{}", x);
}

#[test]
fn lt_test2() {
    let mut foo = Foo(1);
    let f1 = foo.share();
    println!("{:?}", f1);
    let f2 = foo.mutate_and_share();
    //println!("{:?}",f1); 这里不行
    f2.0 = 3;
    println!("{:?}", f2)
}

#[derive(Debug)]
struct Foo(i32);

impl Foo {
    fn mutate_and_share(&mut self) -> &mut Self {
        self
    }
    fn share(&self) -> &Self {
        self
    }
}

#[test]
fn test() {
    let n = 10;
    let x = &n;
    let y = &n;
    do_nothing(x, y);
}

fn do_nothing<'a, 'b: 'a>(x: &'a u32, y: &'b u32) -> &'a u32 {
    x
}
