use crate::Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = Vec::new();
        for dir in path.split('/') {
            match dir {
                "." | "" => continue,
                ".." => {
                    stack.pop();
                }
                _ => {
                    stack.push(dir);
                }
            }
        }

        "/".to_string() + &stack.join("/")
    }
}