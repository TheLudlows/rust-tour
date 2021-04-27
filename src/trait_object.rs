use std::mem;

trait T {
    fn func(&self);
}

struct A;

struct B;

// 分别实现trait函数
impl T for A {
    fn func(&self) {}
}

impl T for B {
    fn func(&self) {}
}

//参数是trait object类型，p是一个DST指针
fn print_trait_object(p: &dyn T) {
    let (data, vtable): (*const usize, *const usize) = unsafe {
        // 使用transmute执行强制类型转换，把p的内部数据取出来
        mem::transmute(p)
    };
    println!("data add: {:p}, vtable add: {:p}", data, vtable);
    unsafe {
        //打印指针v指向的内存区间的值
        println!("vtable指向的内存区域：[0x{:x},{},{},0x{:x}]"
                 , *vtable, *vtable.offset(1), *vtable.offset(2), *vtable.offset(3));
    }
}

#[test]
pub fn run() {
    let a = A {};
    println!("a：{:p}", &a);
    println!("a::func address：{:p}", A::func as *const usize);
    print_trait_object(&a);
}