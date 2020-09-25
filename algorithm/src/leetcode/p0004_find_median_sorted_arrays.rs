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
        while i < nums1.len() || j < nums2.len() {
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
    pub fn find_median_sorted_arrays_2(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        let min_left = (nums1.len() + nums2.len() + 1) / 2;
        let min_right = (nums1.len() + nums2.len() + 2) / 2;
        (findK(&nums1, 0, &nums2, 0, min_left) + findK(&nums1, 0, &nums2, 0, min_right)) as f64 / 2.0
    }
}

fn findK(v1: &Vec<i32>, i: usize, v2: &Vec<i32>, j: usize, k: usize) -> i32 {
    if i >= v1.len() {
        return v2[j + k - 1];
    }
    if j >= v2.len() {
        return v1[i + k - 1];
    }
    if k == 1 {
        return min(v1[i], v2[j]);
    }
    let max1 = if (i + k - 1) < v1.len() { v1[i + k - 1] } else { i32::MAX };
    let max2 = if (j + k - 1) < v2.len() { v2[j + k - 1] } else { i32::MAX };
    return if max1 > max2 {
        findK(v1, i, v2, j + k / 2, k - k / 2)
    } else {
        findK(v1, i + k / 2, v2, j, k - k / 2)
    }
}

#[test]
fn test() {
    let v1 = vec![1, 2];
    let v2 = vec![3, 4];

    let r = Solution::find_median_sorted_arrays_2(v1, v2);

    println!("{}", r)
}
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