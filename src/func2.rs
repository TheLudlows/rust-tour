fn main() {
    let v = vec![];
    let v = add(v);
    println!("{:?}",v)
}
fn add(mut v:Vec<i32>) -> Vec<i32> {
    v.push(20);
    v
}