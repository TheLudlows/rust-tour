#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


impl Rectangle {
    fn area(self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        other.height <= self.height && self.width >= other.width
    }
    fn from(width: u32, height: u32) -> Rectangle {
        Rectangle{
            width,
            height
        }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    rect1.area();
    let rect2 = Rectangle { width: 20, height: 50 };
    println!("{}", rect1.can_hold(&rect2));
    Rectangle::from(21,32);
}
