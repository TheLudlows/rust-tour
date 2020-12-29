use std::mem::needs_drop;

struct Inspector<'a>(&'a u8);

impl<'a> Drop for Inspector<'a> {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

#[test]
fn test() {
    let (b, inspector);
    b = Box::new(1);
    inspector = Inspector(&b);
}
struct Point{
    x: i32,
    y: i32,
}

#[test]
fn test_need_drop() {
    assert_eq!(needs_drop::<Point>(),false);
    assert_eq!(needs_drop::<Box<Point>>(),false);

}