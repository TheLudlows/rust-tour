fn main() {
    let double = |num| num * 2;
    println!("{}", double(10));
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}

struct Cache<T: Fn(u32) -> u32> {
    double: T,
    value: Option<u32>,
}

impl<T> Cache<T> where T: Fn(u32) -> u32 {
    fn new(t: T) -> Cache<T> {
        Cache {
            double: t,
            value: None,
        }
    }
}
