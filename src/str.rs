fn main() {
    let s1 = "rust";
    let s2:& 'static str = "rust";
    let p = s2.as_ptr();
    let len = s1.len();
}