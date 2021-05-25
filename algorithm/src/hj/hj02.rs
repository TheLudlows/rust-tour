fn hj02() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("read err");
    let mut c = String::new();
    std::io::stdin().read_line(&mut c).expect("read err");
    c = c.to_lowercase();
    let bs = c.as_bytes();
    let b = bs[0];
    let mut count = 0;
    s = s.to_lowercase();
    let bs = s.as_bytes();
    for i in 0..bs.len() {
        if b == bs[i] {
            count += 1;
        }
    }
    println!("{}", count);
}

#[test]
fn test() {
    print!("{}{}", b'a', b'A');
    hj02();
}