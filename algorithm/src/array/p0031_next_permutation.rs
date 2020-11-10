use crate::Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut i = n;
        for j in (0..n - 1).rev() {
            if nums[j] < nums[j + 1] {
                i = j;
                break;
            }
        }
        if i == n {
            nums.reverse();
            return;
        }
        for j in (i..n).rev() {
            if nums[j] > nums[i] {
                // swap i, j
                nums.swap(i, j);
                // swap rest ([i+1..])
                nums[i + 1..].reverse();
                break;
            }
        }
    }
}