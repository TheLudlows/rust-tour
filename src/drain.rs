#[test]
fn test_drain() {
    let mut v = vec![1, 2, 3, 4];
    let r: Vec<i32> = v.drain(..).collect();
    println!("{:?}", r);
    println!("{:?}", v);
}