fn main() {
    let s1 = "rust";
    let s2:& 'static str = "rust";
    let p = s2.as_ptr();
    let len = s1.len();
}

#[test]
fn reverse_words() {
    // your code here
    "Test String".to_string();
    let str = "abc 123".to_string();
    str.split(" ").map(|e| e.chars().rev().collect())
        .collect::<Vec<String>>().join(" ");
}