fn main() {
    for v in 1..4 {
        println!("{}", v);
    }
    for v in 1..=4 {
        println!("{}", v);
    }
    println!("{}", (1..3).start);
    println!("{}", (1..3).end);
}
