use std::borrow::Borrow;
use std::ops::Deref;
use std::cmp::Eq;
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

#[test]
fn my_box() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    print(&MyBox::new(String::from("a")));
}

fn print(name: &str) {
    println!("Hello, {}!", name);
}

#[test]
fn drop_test() {
    // 注意a b drop的顺序
    let a = CustomSmartPointer {
        data: String::from("aaa"),
    };
    let b = CustomSmartPointer {
        data: String::from("bbb"),
    };
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[test]
fn owner_test() {
    let a = Box::new("a");
    let b = Box::new("b".to_string());
    let c = *a;
    let d = *b;
    println!("{}", a);
    // println!("{}",b)
    println!("{}", c);
    println!("{}", d);
}

#[test]
fn mut_test() {
    let a: Box<i32> = Box::new(1);
    let x: &i32 = a.borrow();
    let y = a.as_ref();
}


// `T` 的泛型 trait。
trait DoubleDrop<T> {
    // 定义一个调用者的方法，接受一个额外的参数 `T`，但不对它做任何事。
    fn double_drop(self, _: T);
}

// 对泛型的调用者类型 `U` 和任何泛型类型 `T` 实现 `DoubleDrop<T>` 。
impl<T, U> DoubleDrop<T> for U {
    // 此方法获得两个传入参数的所有权，并释放它们。
    fn double_drop(self, _: T) {}
}

struct StructFoo;

#[test]
fn test_trait() {
    let a = StructFoo;
    let b = StructFoo;
    a.double_drop(b);
}

#[test]
fn test_ref() {
    let b = Box::new(10);
    assert_eq!(10, *b.deref());
    assert_eq!(10, *b);
    //assert_eq!(&10,b);
    assert_eq!(&10, b.deref());
    assert_eq!(&10, &*b)
}

#[test]
fn test_box() {
    let mut v1 = vec![1, 2, 3, 4];
    v1.push(5);
    println!("cap should be 8: {}", v1.capacity());

    // 从 Vec<T> 转换成 Box<[T]>，此时会丢弃多余的 capacity
    let b1 = v1.into_boxed_slice();
    let mut b2 = b1.clone();

    let v2 = b1.into_vec();
    println!("cap should be exactly 5: {}", v2.capacity());

    assert!(b2.deref() == v2);

    // Box<[T]> 可以更改其内部数据，但无法 push
    b2[0] = 2;
    // b2.push(6);
    println!("b2: {:?}", b2);

    // 注意 Box<[T]> 和 Box<[T; n]> 并不相同
    let b3 = Box::new([2, 2, 3, 4, 5]);
    println!("b3: {:?}", b3);

    // b2 和 b3 相等，但 b3.deref() 和 v2 无法比较
    assert!(b2 == b3);
    // assert!(b3.deref() == v2);
}