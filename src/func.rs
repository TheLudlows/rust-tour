fn main () {

    say(10);
    println!("result is{}",add(10));
}
fn say(age :u32) {
    println!("age is {}", age);
}
fn add(age:u32) -> u32 {
    return age + 10;
}