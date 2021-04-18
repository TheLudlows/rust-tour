use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![-1; nums1.len()];
        let mut stack:Vec<i32> = vec![];
        let mut map = HashMap::new();
        for i in 0..nums2.len() {
            while !stack.is_empty() && nums2[i] > *stack.last().unwrap(){
                let tmp = stack.pop().unwrap();
                map.insert(tmp,  nums2[i]);
            }
            stack.push(nums2[i]);
        }

        for i in 0..nums1.len(){
            ret[i] = *map.get(&nums1[i]).unwrap_or(&-1);
        }
        ret
    }
}