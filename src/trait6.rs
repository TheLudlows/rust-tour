use std::fmt;

trait OutlinePrint: fmt::Display {
    fn print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
fn main() {
    let f: Box<A+B> = Box::new(|| println!("hi"));
}
trait A {

}
trait B {

}