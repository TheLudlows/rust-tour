use std::convert::Into;

#[test]
fn test() {
    print("aaaaa");
    print("bbbb".to_string());
}
pub fn print(s : impl Into<String>) {
    println!("{}", s.into())
}