use crate::leetcode::common::{Solution, ListNode};
use std::borrow::BorrowMut;

impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut cur = head.as_mut();

        while let Some(ref node) = cur {
            cur = cur.unwrap().next.as_mut();
            len += 1;
        }

        None
    }
}

#[test]
fn test() {
    let tail = Some(Box::new(ListNode::new(3)));
    let mid = Some(Box::new(ListNode{ val: 2, next: tail }));
    let head = Some(Box::new(ListNode{ val: 1, next: mid }));

    Solution::rotate_right(head,2);
}