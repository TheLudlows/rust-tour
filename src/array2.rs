
fn main() {
    let arr = ["a".to_string(),"b".to_string(),"c".to_string()];
    print(arr);
}
fn print(arr:[String;3]) {
    for s in arr.iter() {
        println!("{}",s)
    }
}