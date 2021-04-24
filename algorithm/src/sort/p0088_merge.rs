use crate::Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m as usize;
        let mut j = n as usize;
        let mut k = (m + n - 1) as usize;

        while i > 0 && j > 0 {
            if nums1[i - 1] > nums2[j - 1] {
                nums1[k] = nums1[i - 1];
                i -= 1;
            } else {
                nums1[k] = nums2[j - 1];
                j -= 1;
            }
            k -= 1;
        }
        while j > 0 {
            nums1[k] = nums2[j - 1];
            k -= 1;
            j -= 1;
        }
    }
}

#[test]
fn test() {
    let mut n1 = vec![1, 2, 3, 0, 0, 0];
    let mut n2 = vec![4, 5, 6];
    Solution::merge(&mut n1, 3, &mut n2, 3);
}
