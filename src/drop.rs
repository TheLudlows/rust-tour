use std::mem::needs_drop;

struct Inspector<'a>(&'a u8);

impl<'a> Drop for Inspector<'a> {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

#[test]
fn test() {
    let (b, _inspector);
    b = Box::new(1);
    _inspector = Inspector(&b);
}
struct Point{
    x: i32,
    y: i32,
}

#[test]
fn test_need_drop() {
    assert_eq!(needs_drop::<Point>(),false);
    assert_eq!(needs_drop::<Box<Point>>(),true);

}

struct MyBox(String);

impl Drop for MyBox {
    fn drop(&mut self) {
        println!("dropped");
    }
}

#[test]
fn test_drop() {
    let mut _b1 = MyBox(String::from("1"));
    _b1 = MyBox(String::from("2"));
    println!("line~");
    println!("{}",_b1.0);
}