fn main() {
    println!("Hello, world!");
}


// 解析出来的图片处理的参数
struct ImageSpec {
    specs: Vec<Spec>
}

// 每个参数的是我们支持的某种处理方式
enum Spec {
    Resize(Resize),
    Crop(Crop),
}

// 处理图片的 resize
struct Resize {
    width: u32,
    height: u32
}