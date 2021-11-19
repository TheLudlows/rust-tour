use std::any::{Any, type_name_of_val, TypeId};

fn is_string1(s: &dyn Any) -> bool {
    TypeId::of::<String>() == s.type_id()
}

fn is_string2(s: &dyn Any) -> bool {
    s.is::<String>()
}

#[test]
fn test_type_id() {
    struct Foo {
        x: u8,
        y: u8,
    }
    let f = Foo { x: 0, y: 10 };
    let num = 200;
    let str: &str = "abc";
    println!("{:?}", (&f).type_id());
    println!("{:?}", f.type_id());
    println!("{:?}", str.type_id());
    println!("{:?}", num.type_id());
}

#[test]
fn test_is() {
    let str = &"abc" as &dyn Any;
    let s: &str = "a";
    println!("{}", type_name_of_val(&s));

    println!("{}", type_name_of_val("a"));

    println!("{}", str.is::<&str>());

    assert!(!is_string1(&"a"));
}

#[test]
fn test_cast() {
    print_if_string(&0);

    let mut x = 10u32;
    let mut s = "starlord".to_string();

    modify_if_u32(&mut x);
    modify_if_u32(&mut s);

    assert_eq!(x, 42);
    assert_eq!(&s, "starlord");
}

fn print_if_string(s: &dyn Any) {
    if let Some(string) = s.downcast_ref::<String>() {
        println!("It's a string({}): '{}'", string.len(), string);
    } else {
        println!("Not a string...");
    }
}

fn modify_if_u32(s: &mut dyn Any) {
    if let Some(num) = s.downcast_mut::<u32>() {
        *num = 42;
    }
}
