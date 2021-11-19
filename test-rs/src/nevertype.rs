fn main() {
    let mut i = 10;

    let a = loop {
        if i > 10 {
            break i;
        }
        i += 100;
    };
    println!("{:?}", a)
}