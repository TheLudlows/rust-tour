fn main() {
    let mut v = vec![];
    let mut s = String::new();
    let n = std::io::stdin().read_line(&mut s).unwrap();
    s.pop();
    let mut l = s.parse::<u64>().unwrap();
    let mut i = 2;
    while i <=l {
        while l % i == 0 {
            v.push(i);
            l /= i;
        }
        i += 1;
    }

    for i in v {
        print!("{} ", i);
    }
}

