use std::option::Option::Some;

#[test]
fn main() {
    let x = 10;
    let n = if x > 5 { 100 } else { 200 };
    println!("{}", x);

    for n in 1..10 {
        println!("{}", n);
    }
    let n = 42;
    match n {
        1..=3 => println!("1...3"),
        m @ 42 => println!("{}", m),
        _ => println!("other"),
    }

    let mut arr = vec![1, 2, 3, 4, 5];
    while let Some(x) = arr.pop() {
        println!("{}", x)
    }
}
