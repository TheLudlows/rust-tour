use std::ptr;

#[test]
fn test_ptr() {
    #[derive(Debug)]
    struct Foo{
        a:i32
    }

    let f = Foo{a:10};
    unsafe {
        let r = ptr::read(&f);
        println!("{:p}", &r);
        println!("{:p}", &f);
    }
}