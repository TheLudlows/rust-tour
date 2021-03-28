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
    let data = vec![1, 2, 3];
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


trait DoSomething<T> {
    fn do_sth(&self, value: T);
}

impl<'a, T: Debug> DoSomething<T> for &'a usize {
    fn do_sth(&self, value: T) {
        println!("{:?}", value);
    }
}

fn foo<'a>(b: Box<for<'f> DoSomething<&'f usize>>) {
    let s: usize = 10;
    b.do_sth(&s); // error[E0597]: `s` does not live long enough
}

#[test]
fn test_n() {
    let x = Box::new(&2usize);
    foo(x);
}

#[test]
fn test_lf() {
    let mut data = vec![1, 2, 3];
    let x = &data[0];
    //data.push(1);
    println!("{:?}", x);
}

struct Closure<F> {
    data: (u8, u16),
    func: F,
}

impl<F> Closure<F>
    where for<'a> F: Fn(&'a (u8, u16)) -> &'a u8,
{
    fn call<'a>(&'a self) -> &'a u8 {
        (self.func)(&self.data)
    }
}

fn do_it<'a>(data: &'a (u8, u16)) -> &'a u8 { &data.0 }

#[test]
fn test_clos() {
    let clo = Closure { data: (0, 1), func: do_it };
    println!("{}", clo.call());
}

struct Foo1<'a> {
    a: u32,
    b: Option<&'a u32>,
}

/*impl <'a> Drop for Foo1<'a> {
    fn drop(&mut self) {
        //todo!()
    }
}*/

#[test]
fn test_foo1() {
    let mut f = Foo1 {
        a: 10,
        b: None,
    };
    f.b = Some(&f.a);
}

