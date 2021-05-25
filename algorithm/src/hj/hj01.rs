fn hj01() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Failed to read line");
    let arr = s.split_ascii_whitespace().collect::<Vec<&str>>();
    println!("{}", arr[arr.len() - 1].len());
}

#[test]
fn test() {
    hj01();
}