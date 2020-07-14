use compressed_vec::VectorU32Appender;
mod compresion;
mod json;


struct Foo;
type Boo =  Foo;
impl Foo{
    pub fn say_hello(&self) {
        println!("hello")
    }
}
fn main() {

    let b = Boo;
    b.
}
