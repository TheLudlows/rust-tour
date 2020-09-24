use crate::leetcode::common::Solution;
use crate::datastruct::SimpleLinkedList::Node;
use std::cmp::min;

impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        let min_left = (nums1.len() + nums2.len() + 1) / 2;
        let min_right = (nums1.len() + nums2.len() + 2) / 2;
        let mut res: f64 = 0f64;

        let mut i = 0;
        let mut j = 0;
        let mut cur: i32 = 0;
        while  i < nums1.len() || j < nums2.len() {
            // 如果第二个数组已经遍历完，只能选择第一个数组
            if j == nums2.len() || (i < nums1.len() && nums1[i] < nums2[j]) {
                cur = nums1[i];
                i += 1;
            } else {
                cur = nums2[j];
                j += 1;
            }

            if i + j == min_left {
                res += cur as f64;
            }
            if i + j == min_right {
                res += cur as f64;
            }
        }
        res / 2.0
    }
}

#[test]
fn test() {
    let v1 = vec![1, 2];
    let v2 = vec![3, 4];

    let r = Solution::find_median_sorted_arrays(v1, v2);

    println!("{}", r)
}
/// JAVA
// class Solution {
//     public double findMedianSortedArrays(int[] nums1, int[] nums2) {
//         int len1 = nums1.length;
//         int len2 = nums2.length;
//         int total = len1 + len2;
//         int left = (total + 1) / 2;
//         int right = (total + 2) / 2;
//         return (findK(nums1, 0, nums2, 0, left) + findK(nums1, 0, nums2, 0, right)) / 2.0;
//
//     }
//
//     //找到两个数组中第k小的元素
//     public int findK(int[] nums1, int i, int[] nums2, int j, int k) {
//         if (i >= nums1.length)
//             return nums2[j + k - 1];
//         if (j >= nums2.length)
//             return nums1[i + k - 1];
//         if (k == 1) {
//             return Math.min(nums1[i], nums2[j]);
//         }
//         //计算出每次要比较的两个数的值，来决定 "删除"" 哪边的元素
//         int mid1 = (i + k / 2 - 1) < nums1.length ? nums1[i + k / 2 - 1] : Integer.MAX_VALUE;
//         int mid2 = (j + k / 2 - 1) < nums2.length ? nums2[j + k / 2 - 1] : Integer.MAX_VALUE;
//         //通过递归的方式，来模拟删除掉前K/2个元素
//         if (mid1 < mid2) {
//             return findK(nums1, i + k / 2, nums2, j, k - k / 2);
//         }
//         return findK(nums1, i, nums2, j + k / 2, k - k / 2);
//     }
// }