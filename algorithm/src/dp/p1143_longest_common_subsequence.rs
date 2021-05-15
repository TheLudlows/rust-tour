use crate::Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut memo = vec![vec![-1;text2.len()+1];text1.len()+1];
        find(text1.as_bytes(), text2.as_bytes(), text1.len(), text2.len(), &mut memo)
    }
}

pub fn find(t1: &[u8], t2: &[u8], idx1: usize, idx2: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
    if idx1 == 0 || idx2 == 0 {
        return 0;
    }
    if memo[idx1][idx2] != -1 {
        return memo[idx1][idx2];
    }
    let mut ret = 0;
    if t1[idx1 - 1] == t2[idx2 - 1] {
        ret = std::cmp::max(ret, 1 + find(t1, t2, idx1 - 1, idx2 - 1, memo));
    } else {
        ret = std::cmp::max(find(t1, t2, idx1, idx2 - 1, memo), find(t1, t2, idx1 - 1, idx2,memo));
    }
    memo[idx1][idx2] = ret;
    ret
}
#[test]
fn test() {
    let text1 = "abcde".to_string();
    let text2 = "ace".to_string();
    let r = Solution::longest_common_subsequence(text1, text2);
    println!("{}", r);
}