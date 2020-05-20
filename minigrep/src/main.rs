use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let a1 = &args[1];
    let a2 = &args[2];
    let filename = &args[3];
    println!("a1:{},a2:{}", a1, a2);

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
}
