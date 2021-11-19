use std::convert::{From, TryFrom};

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
fn test_from() {
    let n = 5;
    // 试试删除类型说明
    let num: Number = n.into();
    let n = Number::from(n);
    println!("My number is {:?}", num);
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

#[test]
fn test_try() {
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
}
