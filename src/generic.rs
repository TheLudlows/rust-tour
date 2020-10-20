use std::cell::{Ref, RefCell};

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

trait Double<T> {
    fn double(&self) -> Option<T>;
}

struct S {
    data: i32,
}

impl Double<i32> for S {
    fn double(&self) -> Option<i32> {
        unimplemented!()
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.0, y: 10.0 };
    println!("p.x = {}", p1.x());
    println!("distance_from_origin = {}", p2.distance_from_origin());
}
