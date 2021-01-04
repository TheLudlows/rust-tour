use crate::Solution;

impl Solution {
    pub fn interpret(command: String) -> String {
        let command = command.as_bytes();
        let mut vec = vec![];
        for i in 0..command.len() {
            let b = command[i];
            if b == b'G' {
                vec.push(b'G');
            }else if b == b')' {
                if i > 0 && command[i-1] == b'(' {
                    vec.push(b'o');
                } else if i > 2 && command[i-1] == b'l' && command[i-2] == b'a' && command[i-3] == b'(' {
                    vec.push(b'a');
                    vec.push(b'l');
                }
            }
        }
        String::from_utf8(vec).unwrap()
    }
}