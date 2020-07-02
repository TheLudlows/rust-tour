use std::borrow::Cow;

fn main() {
    let mut c = Cow::from("");
    let v = c.to_mut();

}
