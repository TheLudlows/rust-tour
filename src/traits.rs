use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self::Output) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

#[test]
fn func_test() {
    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

#[test]
fn func_test2() {
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

struct Pig;

struct Duck;

trait Fly {
    fn fly(&self) -> bool;
}

impl Fly for Pig {
    fn fly(&self) -> bool {
        false
    }
}

impl Fly for Duck {
    fn fly(&self) -> bool {
        true
    }
}

fn fly_static<T: Fly>(t: &T) -> bool {
    t.fly()
}

fn fly_dyn(t: &dyn Fly) -> bool {
    t.fly()
}

#[test]
fn func3_test() {
    let pig = Pig;
    let duck = Duck;
    assert_eq!(fly_dyn(&pig), false);
    assert_eq!(fly_dyn(&duck), true);
    assert_eq!(fly_static(&pig), false);
    assert_eq!(fly_static(&duck), true);
}

trait star {
    fn sing(&self);
    fn dance(&self);
    fn rap(&self) {
        println!("rap")
    }
    fn basketball(&self) {
        println!("🏀");
    }
}

#[derive(Clone)]
struct cxk;

/// 默认方法可以不实现
impl star for cxk {
    fn sing(&self) {
        println!("cxk sing")
    }

    fn dance(&self) {
        unimplemented!()
    }
}

fn to_sing1<S>(s: S) where S: star {
    s.sing();
}

fn to_sing2(s: &dyn star) {
    s.sing();
}

fn to_sing3(s: impl star) {
    s.sing();
}

fn to_sing4(s: &impl star) {
    s.sing();
}

#[test]
fn test() {
    let cxk = cxk;
    to_sing1(cxk.clone());
    to_sing2(&cxk.clone());
    to_sing3(cxk.clone());
    to_sing4(&cxk.clone());
}
