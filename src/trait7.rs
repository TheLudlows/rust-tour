struct Pig;
struct Duck;
trait Fly{
    fn fly(&self) -> bool;
}
impl Fly for Pig{
    fn fly(&self) -> bool {
        false
    }
}

impl Fly for Duck{
    fn fly(&self) -> bool {
        true
    }
}

fn fly_static<T: Fly>(t:T) -> bool{
    t.fly()
}
fn fly_dyn(t:& dyn Fly) -> bool{
    t.fly()
}
fn main() {
    let pig = Pig;
    let duck = Duck;
    assert_eq!(fly_dyn(pig),false);
}