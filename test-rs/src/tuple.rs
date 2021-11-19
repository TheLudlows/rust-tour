fn main() {
    let t = (1, 2, "a");
    assert_eq!(t.0, 1);
    assert_eq!(t.2, "a");
    let tp = (4, );
    assert_eq!(t.0, 4);
}
