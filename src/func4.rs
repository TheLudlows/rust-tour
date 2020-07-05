fn main() {
    let f = Foo { i: 32 };
    print(f);
    //println!("{:?}", f)
}

#[derive(Debug)]
struct Foo { i: i32 }

fn print(ref mut f: Foo) {
    println!("{:p}", f);
}