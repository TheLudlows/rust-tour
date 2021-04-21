use crate::{ListNode, Solution};

impl Solution {
    pub fn kth_to_last(head: Option<Box<ListNode>>, k: i32) -> i32 {
        let mut fast = head.as_ref();
        let mut slow = head.as_ref();
        for _ in 0..k {
            fast = fast.unwrap().next.as_ref();
        }

        while fast.is_some() {
            fast = fast.unwrap().next.as_ref();
            slow = slow.unwrap().next.as_ref();
        }
        slow.unwrap().val
    }
}