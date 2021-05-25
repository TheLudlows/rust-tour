
fn main() {
    let mut v = vec![];
    loop {
        let mut s = String::new();
        let n = std::io::stdin().read_line(&mut s).unwrap();
        if n <= 0 {
            break;
        }
        for ss in s.split_ascii_whitespace() {
            let str = ss.to_lowercase();
            let arr = str.split_at(2);
            let r = i32::from_str_radix(arr.1, 16).unwrap();
            v.push(r);
        }
    }
    for i in v {
        println!("{}", i);
    }
}

#[test]
fn test() {
    // 不能0x开头
    let s = "0x11".to_string();

    println!("{:?}", s.split_at(2));
    let r = i32::from_str_radix("AA", 16).unwrap();
}