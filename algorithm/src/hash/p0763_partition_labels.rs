use std::cmp::max;

use crate::Solution;

///  映射出每个字符的最晚结束位置，遍历数组，维护一个窗口，每次取当前包含字符的最晚结束，当i和结束相等，划分出一个分区。
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut d = [0; 26];
        for (i, c) in s.bytes().enumerate() {
            d[(c - b'a') as usize] = i
        }
        let mut ans = vec![];
        let (mut l, mut r) = (0, 0);
        for (i, c) in s.bytes().enumerate() {
            r = max(d[(c - b'a') as usize], r);
            if i == r {
                ans.push((r + 1 - l) as i32);
                l = i + 1
            }
        }
        ans
    }
}

#[test]
fn test() {
    let v = "abcd".to_string();
    println!("{:?}", Solution::partition_labels(v));
}
