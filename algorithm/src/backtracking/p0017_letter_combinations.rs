use crate::Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let v = vec!["", " ", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut ret = vec![];
        if digits.is_empty() {
            return ret
        }
        let digits = digits.chars().collect::<Vec<char>>();
        let mut cur = String::new();
        find(&digits, 0, &v, &mut ret, &mut cur);
        ret
    }
}

pub fn find(digits: &Vec<char>, idx: usize, table: &Vec<&str>, ret: &mut Vec<String>, cur: &mut String) {
    if idx == digits.len() {
        ret.push(cur.clone());
        return
    }
    for c in table[digits[idx].to_digit(10).unwrap() as usize].chars() {
        cur.push(c);
        find(digits, idx + 1, table, ret, cur);
        cur.pop();
    }
}

#[test]
fn test_toi32() {
    let s = "123".chars().collect::<Vec<char>>();
    let r = s[0].to_digit(10).unwrap();
    println!("{}", r);
}