
trait Man{
    fn fuck(&self);
}
struct Me;
impl Man for Me {
    fn fuck(&self) {
        println!("fuck")
    }
}
fn main () {
    let me = Me;
}

struct traitObject{
    pub data: *mut (),
    pub vtable: *mut ()
}