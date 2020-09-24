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
        loop {
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

            if i == nums1.len() && j == nums2.len() {
                break;
            }
        }
        res / 2.0
    }

    pub fn find_median_sorted_arrays_2(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        let min_left = (nums1.len() + nums2.len() + 1) / 2;
        let min_right = (nums1.len() + nums2.len() + 2) / 2;
        let mut res: f64 = 0f64;

        let mut i = min(nums1.len()-1,);
        let mut j = 0;
        let mut cur: i32 = 0;
        loop {
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

            if i == nums1.len() && j == nums2.len() {
                break;
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