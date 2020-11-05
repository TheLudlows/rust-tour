use std::vec::Vec;
macro_rules! unless {
    ($arg:expr, $branch:expr) => {
        if !$arg {
            $branch
        };
    };
}

#[macro_export]
macro_rules! vector {
    ($($x:expr),*) => {
    {
        let mut v = Vec::new();
        $(v.push($x);)*
        v
    }
    };
}

#[test]
fn main() {
    let a = 2;
    let b = 3;
    unless!(a > b, println!("a<=b"));
    let v = vector![1,2,3];
    let v2 = vector!(1,2,3);

    println!("{:?}{:?}", v,v2)
}
