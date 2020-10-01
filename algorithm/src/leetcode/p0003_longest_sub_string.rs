use crate::leetcode::common::Solution;
use crate::Solution;

/// 双指针维护一个窗口，i表示将要加入的元素，如果i存在于窗口，窗口可以用set表示，便于查询。
/// 则更改窗口的起始地址，如果不存在则扩大窗口的结束地址

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let mut max_len = 0;
        let ss = s.as_bytes();
        let mut s_index: usize = 0;
        let mut e_index: usize = 0;

        let mut i = 1;
        while i < s.len() {
            let index = indexOf(ss, ss[i], s_index, e_index);
            e_index += 1;
            if index == -1 {
                if e_index - s_index > max_len {
                    max_len = e_index - s_index;
                }
            } else {
                s_index = index as usize + 1;
            }
            i += 1;
        }
        (max_len + 1) as i32
    }
}

fn indexOf(arr: &[u8], c: u8, start: usize, end: usize) -> i32 {
    for i in start..=end {
        if arr[i] == c {
            return i as i32;
        }
    }
    -1
}

#[test]
fn test() {
    let n = Solution::length_of_longest_substring("acbae".to_string());
    println!("{}", n);
}
