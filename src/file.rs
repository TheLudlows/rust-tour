use std::env::current_dir;
use std::fs;

#[test]
fn test_dir() {
    let mut ps =  current_dir().unwrap().read_dir().unwrap().into_iter()
        .map(|p| p.unwrap().file_name().to_str().unwrap().to_string())
        .filter(|name|name.ends_with(".log"))
        .filter_map(|name| name.split_at(name.len() - 4).0.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    ps.sort();
    println!("{:?}",ps);
}

#[test]
fn test_join() {
    let p = current_dir().unwrap().join("a");
    println!("{:?}",p);
}