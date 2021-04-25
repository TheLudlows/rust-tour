use bytes::Bytes;

#[test]
fn test_bytes() {
    let mut b = Bytes::new();
    println!("{}", std::mem::size_of_val(&b));
    println!("{}", std::mem::align_of_val(&b));

    println!("{}", std::mem::size_of::<Vec<usize>>());

}