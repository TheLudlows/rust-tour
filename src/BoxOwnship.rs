fn main() {
    let a = Box::new("a");
    let b = Box::new("b".to_string());
    let c = *a;
    let d = *b;
    println!("{}",a);
    // println!("{}",b)
    println!("{}",c);
    println!("{}",d);
}