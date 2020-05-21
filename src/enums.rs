fn main() {
    #[derive(Debug)]
    enum enum_ {
        a,
        b(String),
        s { num: u32, name: String },
    }
    println!("{:?}", enum_::a);
    println!("{:?}", enum_::b(String::from("b")));

    let e = enum_::b(String::from("four"));

    let num: Option<u32> = Some(10);
    match num {
        Some(t) => println!("T is {}", t),
        None => println!("no")
    };
    match e {
        enum_::a => println!("a"),
        enum_::b(str) => println!("{}", str),
        enum_::s { num, name } => println!("s")
    }

    let n = Some(1);
    if let x = n {
        println!("{}",x.unwrap())
    }
}

