use std::cmp::Ordering::*;
use std::collections::HashMap;

use crate::Solution;
/// todo
impl Solution {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort_unstable();
        nums2.sort_unstable();
        let mut ans = Vec::with_capacity(100);
        if let (Some(mut n1), Some(mut n2)) = (nums1.pop(), nums2.pop()) {
            loop {
                match n1.cmp(&n2) {
                    Greater => n1 = if let Some(x) = nums1.pop() { x } else { break },
                    Less => n2 = if let Some(x) = nums2.pop() { x } else { break },
                    _ => {
                        ans.push(n1);
                        if let (Some(nn1), Some(nn2)) = (nums1.pop(), nums2.pop()) {
                            n1 = nn1;
                            n2 = nn2
                        } else { break }
                    }
                }
            }
        }
        (&mut ans).reverse();
        ans
    }
}

