use std::ops::Mul;

fn main() {
    let z = square(100,200);
    let r = square::<f32>(4.2,5.0);
}

fn square<T: Mul<T,Output=T>>(x: T, y: T) -> T {
    x * y
}