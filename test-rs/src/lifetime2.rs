struct Objects<'a> {
    value: Vec<String>,
    temp: Vec<&'a String>,
}

impl<'a> Objects<'a> {
    fn my_change(&'a mut self) {
        for v in &self.value {
            self.temp.push(v);
        }
    }
}
struct ByteIter<'a> {
    remainder: &'a [u8],
}
impl<'a> ByteIter<'a> {
    fn next(&mut self) -> Option<&'a u8> {
        if self.remainder.is_empty() {
            None
        } else {
            let byte = &self.remainder[0];
            self.remainder = &self.remainder[1..];
            Some(byte)
        }
    }
}
#[test]
fn test() {

    let mut bytes = ByteIter { remainder: b"123" };
    assert_eq!(Some(&b'1'), bytes.next());
    assert_eq!(Some(&b'2'), bytes.next());
    assert_eq!(Some(&b'3'), bytes.next());

    assert_eq!(None, bytes.next());
}

#[derive(Debug)]
struct NumRef<'a>(&'a i32);

impl<'a> NumRef<'a> {
    // 我定义的泛型结构体以 'a 为参数，这意味着我也需要给方法的参数
    // 标注为 'a 生命周期，对吗？（答案：错）
    fn some_method(& mut self) {}
}
#[test]
fn main2() {
    let mut num_ref = NumRef(&5);
    num_ref.some_method(); // 可变借用 num_ref 直至其生命周期结束
    num_ref.some_method(); // 编译错误
    println!("{:?}", num_ref); // 同样编译错误
}