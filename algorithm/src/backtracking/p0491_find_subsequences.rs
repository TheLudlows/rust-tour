use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn find_subsequences(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut cur = vec![];
        let mut set = HashSet::new();
        let mut ret = vec![];
        find(&nums, &mut cur, &mut ret, 0, &mut set);
        ret
    }
}

fn find(nums: &Vec<i32>, cur: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>, idx: usize, set: &mut HashSet<Vec<i32>>) {
    if idx > nums.len() {
        return
    }
    if cur.len() > 1 {
        if !set.contains(cur) {
            set.insert(cur.clone());
            ret.push(cur.clone());
        }
    }

    for i in idx..nums.len() {
        if cur.is_empty() || cur[cur.len() - 1] <= nums[i] {
            cur.push(nums[i]);
            find(nums, cur, ret, i + 1, set);
            cur.pop();
        }
    }
}

#[test]
fn test() {
    let v = vec![4, 6, 7, 7];
    let r = Solution::find_subsequences(v);
    println!("{:?}", r);
}