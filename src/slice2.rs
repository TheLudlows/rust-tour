fn main() {
    let arr = [1,2,3,4];
    assert_eq!(&arr[1..],[2,3,4]);
    assert_eq!(&arr[..],arr);
    println!("{}",(&arr[1..]).len())
}