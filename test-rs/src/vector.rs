

// see small vec mini vec


#[test]
fn main() {
    let mut v = Vec::new();
    //let v = vec![1, 2, 3];
    v.push(100);
    println!("{}", v.pop().unwrap());

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];
    //let first = &v[0];
    v.push(6);
    println!("The first element is: {:?}", v);

    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
    }
    let mut v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    let i = v[0];
    v.push(200);

    v[1] = 2;
    //let x = v[0..1];
}
#[test]
fn test() {
    let v = vec!{1,2,3};
}
