struct Vertex {
    x: u8,
    y: i32,
    //b: Box<i32>
}

#[repr(C)]
struct Vertex2 {
    x: u8,
    y: i32,
    //b: Box<i32>
}

fn to_bytes<T>(t: &T) -> &[u8]{
    let p: *const T = t;
    let p = p as *const u8;
    unsafe{
        std::slice::from_raw_parts(p, std::mem::size_of::<T>())
    }
}
#[test]
fn main() {
    let v = Vertex{x: 0, y: 0, /*b: Box::new(0)*/};
    let v2 = Vertex2{x: 0, y: 0, /*b: Box::new(0)*/};
    println!("{:?}", to_bytes(&v));
    println!("{:?}", to_bytes(&v2));
}