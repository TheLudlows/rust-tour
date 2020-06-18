trait Add<RHS, Output> {
    fn add(self, r: RHS) -> Output;
}


impl Add<i32, i32> for i32 {
    fn add(self, r: i32) -> i32 {
        self + r
    }
}

impl Add<u32, u32> for u32 {
    fn add(self, r: u32) -> u32 {
        self + r
    }
}

fn main() {
    let a: i32 = 1;
    let b: i32 = 2;
    let c: u32 = 3;
    let d: u32 = 4;
    let e = a.add(b);
    let f = c.add(d);
    assert_eq!(e, 3);
    assert_eq!(f, 7);
}