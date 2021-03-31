use std::ptr::Unique;

#[test]
 fn test() {
    let mut p = 10;
    let u = Unique::new(&mut p).unwrap();
    println!("{:?}", u);
    println!("{:p}", &p);
    let p = print as usize;
    unsafe {
        let u: Unique<i32> = Unique::dangling();
        let u: Unique<i32> = Unique::dangling();

        println!("{:?}", u);
        println!("{:?}", *u.as_ref())
    }
}

fn print(a: *const i32, b: &i32) {
    //yield 1;
}
