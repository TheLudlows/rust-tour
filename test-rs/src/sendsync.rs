fn main() {
    let x = vec!["a"];
    let h = std::thread::spawn(|| x);

    let y = String::from("a");
    let h = std::thread::spawn(|| y);

    let z = 32;
    let h = std::thread::spawn(move || z);
}
