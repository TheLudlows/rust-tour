use crate::Solution;

impl Solution {
    pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
        // 特殊用例
        if nums.len() <= 1 {
            return 0;
        }
        let mut res = 0;
        let len = nums.len();
        mergeSort(&mut nums, 0, len-1, &mut res);
        res
    }
}

fn mergeSort(nums: &mut Vec<i32>, start: usize, end: usize, res: &mut i32) {
    if start >= end {
        return;
    }
    let mid = (start + end) / 2;
    mergeSort(nums, start, mid, res);
    mergeSort(nums, mid + 1, end, res);
    merge(nums, start, end, res);
}

fn merge(nums: &mut Vec<i32>, left: usize, right: usize, res: &mut i32) {
    // 临时数组
    let mut nums_copy: Vec<i32> = vec![];
    let mid = (left + right) / 2;
    let (mut i, mut j) = (left, mid + 1);
    while i <= mid && j <= right {
        if nums[i] <= nums[j] {
            nums_copy.push(nums[i]);
            i += 1;
        } else {
            nums_copy.push(nums[j]);
            j += 1;
            // 和归并唯一不同的地方
            *res += (mid - i + 1) as i32;
        }
    }
    while i <= mid {
        nums_copy.push(nums[i]);
        i += 1;
    }
    while j <= right {
        nums_copy.push(nums[j]);
        j += 1;
    }

    for i in 0..nums_copy.len() {
        nums[left + i] = nums_copy[i];
    }

}
#[test]
fn test() {
    Solution::reverse_pairs(vec![7,5,6,4]);
}