use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

#[test]
fn main() {
    let int = 5;
    // 试试删除类型说明
    let num: Number = int.into();
    let n = Number::from(int);
    println!("My number is {:?}", num);
}
