fn main() {
    let mut v = vec![];
    loop {
        let mut line = String::new();
        let read = std::io::stdin().read_line(&mut line).unwrap();
        if read == 0 {
            break;
        }
        // 空格
        line.pop();
        if line.len() < 8 {
            while line.len() < 8 {
                line.push('0')
            }
            v.push(line);
        }else {
            while line.len() % 8 != 0 {
                line.push('0');
            }
            let mut i = 0;
            while i < line.len() {
                v.push(String::from(&line[i..i+8]));
                i+=8;
            }
        }

    }
    for s in v.iter() {
        println!("{}", s);
    }
}