use std::cmp::min;

use crate::Solution;

/// 双指针
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
        (findK(&nums1, 0, &nums2, 0, min_left) + findK(&nums1, 0, &nums2, 0, min_right)) as f64
            / 2.0
    }
}

fn findK(v1: &Vec<i32>, i: usize, v2: &Vec<i32>, j: usize, k: usize) -> i32 {
    // 当nums1 删除完，则直接返回j+k-1位置的数字，nums2删完同理
    if i >= v1.len() {
        return v2[j + k - 1];
    }
    if j >= v2.len() {
        return v1[i + k - 1];
    }
    // k==1时表示找最小的数字
    if k == 1 {
        return min(v1[i], v2[j]);
    }
    let max1 = if (i + k / 2 - 1) < v1.len() {
        v1[i + k / 2 - 1]
    } else {
        i32::MAX
    };
    let max2 = if (j + k / 2 - 1) < v2.len() {
        v2[j + k / 2 - 1]
    } else {
        i32::MAX
    };
    return if max1 > max2 {
        findK(v1, i, v2, j + k / 2, k - k / 2)
    } else {
        findK(v1, i + k / 2, v2, j, k - k / 2)
    };
}

#[test]
fn test() {
    let v1 = vec![1, 2];
    let v2 = vec![3, 4, 5, 6];

    let r = Solution::find_median_sorted_arrays_2(v1, v2);

    println!("{}", r)
}
