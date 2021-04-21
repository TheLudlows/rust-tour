use crate::Solution;

impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let mut result = String::new();

        let mut index = 0;
        let bytes = s.as_bytes();

        while index < bytes.len() {
            if index + 2 < bytes.len() && bytes[index + 2] == b'#' {
                result.push((96 + (bytes[index] - 48) * 10 + (bytes[index + 1] - 48)) as char);
                index += 3;
            } else {
                result.push((96 + bytes[index] - 48) as char);
                index += 1;
            }
        }
        result
    }
}

#[test]
fn test() {
    println!("{}", '1' as u8)
}