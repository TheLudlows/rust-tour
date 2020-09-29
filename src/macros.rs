macro_rules! unless {
    ($arg:expr, $branch:expr) => (if !$arg { $branch };);
}

#[test]
fn main() {
    let a = 2;
    let b = 3;
    unless!(a > b, println!("a<=b"));
}