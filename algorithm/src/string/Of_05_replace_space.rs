use crate::Solution;

impl Solution {
    pub fn replace_space(s: String) -> String {
        let v = s.chars();
        let mut result = String::new();
        for i in v.into_iter() {
            if i == ' ' {
                result.push_str("%20");
            } else {
                result.push(i);
            }
        }
        s
    }
}
#[test]
fn test() {}
