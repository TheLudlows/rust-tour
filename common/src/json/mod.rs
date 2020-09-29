pub struct Foo;

impl Foo {
    pub fn say_hello(&self) {
        println!("hello")
    }
    pub fn new() -> Self {
        Foo
    }
}

pub type Boo = Foo;