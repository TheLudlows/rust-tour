use crate::leetcode::common::Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut v1 = nums1;
        let mut v2 = nums2;
        v1.append(&mut v2);
        v1.sort();
        let len = v1.len();
        return if len % 2 == 0 {
            ((v1[len / 2] + v1[len / 2 - 1]) as f64 / 2.0) as f64
        } else {
            v1[len / 2] as f64
        }
    }
}
#[test]
fn test() {

}