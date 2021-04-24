use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::Solution;

impl Solution {
    // 堆思想
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut heap = BinaryHeap::with_capacity(k);
        for n in nums {
            if heap.len() < k {
                heap.push(Reverse(n));
            } else {
                if n < heap.peek().unwrap().0 {
                    heap.pop();
                    heap.push(Reverse(n));
                }
            }
        }
        heap.pop().unwrap().0
    }
}

impl Solution {
    pub fn find_kth_largest2(mut nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        return quick_sort(&mut nums, 0, len-1, len - k as usize)
    }
}
pub fn quick_sort(arr: &mut Vec<i32>, l: usize, r: usize,  k: usize) -> i32 {
    let pivot = partition(arr, l, r);
    if pivot == k {
        return arr[pivot]
    } else if pivot > k {
        return quick_sort(arr, l, pivot - 1, k);
    } else {
        return quick_sort(arr, pivot + 1, r, k);
    }

}

fn partition(arr: &mut Vec<i32>, l: usize, r: usize) -> usize {
    let mut j = l+1;
    let p = arr[l];

    for i in l..=r {
        if arr[i] < p {
            arr.swap(i,j);
            j+=1;
        }
    }
    arr.swap(l, j-1);
    j-1
}

