fn main() {
    let mut v = vec![];
    add(&mut v);
    println!("{:?}",v)
}

fn add(v:&mut Vec<i32>){
    v.push(20);
}