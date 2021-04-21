use crate::{ListNode, Solution};

/// cloned
/// 双指针
impl Solution {
    pub fn get_kth_from_end(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let (mut fast, mut slow) = (head.as_ref(), head.as_ref());
        for _ in 0..k {
            fast = fast.unwrap().next.as_ref();
        }
        while fast.is_some() {
            fast = fast.unwrap().next.as_ref();
            slow = slow.unwrap().next.as_ref();
        }
        slow.cloned()
    }
}
