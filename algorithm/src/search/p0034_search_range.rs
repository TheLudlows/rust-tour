use crate::Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }
        let mut l = 0;
        let mut r = nums.len() - 1;
        let (mut i, mut j) = (-1, -1);
        while l <= r {
            let m = (l + r) / 2;
            if target == nums[m] {
                i = m as i32;
                j = m as i32;
            }
            if target >= nums[m] {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        if i == -1 {
            return vec![-1, -1];
        }
        while i >= 0 && nums[i as usize] == target {
            i -= 1;
        }
        while j <= (nums.len() - 1) as i32 && nums[j as usize] == target {
            j += 1;
        }
        i += 1;
        j -= 1;
        vec![i, j]
    }
}

#[test]
fn test() {
    let v = vec![5, 7, 7, 8, 8, 10];
    let r = Solution::search_range(v, 8);
    println!("{:?}", r);
}