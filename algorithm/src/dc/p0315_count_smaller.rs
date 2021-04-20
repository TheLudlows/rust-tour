use crate::Solution;

// 归并排序思想
impl Solution {
    pub fn count_smaller(mut nums: Vec<i32>) -> Vec<i32> {
        let mut count = vec![0; nums.len()];
        mergeSort(&mut nums, 0, 0, &mut count);
        count
    }
}

fn mergeSort(nums: &mut Vec<i32>, left: usize, right: usize, count: &mut Vec<i32>) {
    if left >= right {
        return
    }
    let mid = (left + right) / 2;
    mergeSort(nums, right, mid, count);
    mergeSort(nums, mid + 1, right, count);
    merge(nums, left, right, count)
}

fn merge(nums: &mut Vec<i32>, left: usize, right: usize, count: &mut Vec<i32>) {
    let mid = (left + right) / 2;
    let mut tmp = vec![];
    let (mut i, mut j) = (left, mid + 1);

    while i <= mid && j <= right {
        if nums[i] <= nums[j] {
            tmp.push(nums[i]);
            i += 1;
        } else {
            tmp.push(nums[j]);
            j += 1;
            for k in i..=mid {
                count[k] += 1;
            }
        }
    }
}