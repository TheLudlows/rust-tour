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
    inspector;
    println!("last")
}