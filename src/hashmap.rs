
#![allow(unused_variables)]
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);


    let field_name = String::from("Favorite color");
    let mut map = HashMap::new();
    map.insert(field_name, "Blue");
    //println!("{}",field_name)

    let  o = map.get(&("Favorite color".to_string()));
    println!("{}",o.unwrap());

    for (k,v) in map{
        println!("{},{}", k,v)
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    let v = scores.entry(String::from("Yellow")).or_insert(50);
    *v += 100;
    println!("{}", scores.get(&String::from("Yellow")).unwrap());

}