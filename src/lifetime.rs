use std::fmt::Debug;

#[derive (Debug)]
struct Ref<'a,T:'a>(&'a T);
fn print_ref<'a, T>(t:&'a T) where T:'a + Debug{
    println!("{:?}",t)
}
fn print<T>(t:T) where T:Debug{
    println!("{:?}",t)
}
fn main() {

    let r = Ref(&1);
    print_ref(&r);
    print(r);
    let a : &str = "a";
    let b = a.to_string();
}