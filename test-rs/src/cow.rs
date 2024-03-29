use std::borrow::Cow;
use std::borrow::Borrow;
fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
        println!("{:?}", input.is_owned());
    }
}

#[test]
fn main() {
    let arr = [0, 1, 2];
    let mut input = Cow::from(&arr[..]);
    abs_all(&mut input);

    let slice = [-1, 0, 1];
    let mut input = Cow::from(&slice[..]);
    abs_all(&mut input);

    let mut input = Cow::from(vec![-1, 0, 1]);
    abs_all(&mut input);
}
