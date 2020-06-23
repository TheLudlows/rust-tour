fn main() {
    let s = String::from("aaa");
    s.clone();

}
struct stru;
impl Clone for stru {
    fn clone(&self) -> Self {
        stru
    }
}
