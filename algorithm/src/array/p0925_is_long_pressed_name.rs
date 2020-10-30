use crate::Solution;

impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let name = name.as_bytes();
        let typed = typed.as_bytes();
        let mut i = 0;
        let mut j = 0;
        while j < typed.len() {
            if i < name.len() && name[i] == typed[j] {
                i += 1;
                j += 1;
            } else if i > 0 && typed[j] == name[i - 1] {
                j += 1;
            } else {
                return false;
            }
        }
        i == name.len()
    }
}
