#[test]
fn enums() {
    #[derive(Debug)]
    enum Enum {
        A,
        B(String),
        S { num: u32, name: String },
    }
    println!("{:?}", Enum::A);
    println!("{:?}", Enum::B(String::from("b")));

    let e = Enum::B(String::from("four"));

    let num: Option<u32> = Some(10);
    match num {
        Some(t) => println!("T is {}", t),
        None => println!("no"),
    };
    match e {
        Enum::A => println!("a"),
        Enum::B(str) => println!("{}", str),
        Enum::S { num, name } => println!("s"),
    }
}

