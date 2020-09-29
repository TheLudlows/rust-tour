use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::leetcode::common::{ListNode, Solution};

impl PartialOrd<ListNode> for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

/// 最小堆的使用
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut res = Box::new(ListNode::new(0));
        if lists.len() == 0 {
            return None;
        }
        let mut minHeap = BinaryHeap::new();
        for mut list in lists {
            if list.is_some() {
                minHeap.push(list.take()?); // Storing as Option<Box<ListNode>>
            }
        }
        let mut cur = &mut res;
        while !minHeap.is_empty() {
            cur.next = minHeap.pop(); // Return as Option<Box<ListNode>>
            cur = cur.next.as_mut()?;
            if cur.next.is_some() {
                minHeap.push(cur.next.take()?);
            }
        }
        res.next
    }
}