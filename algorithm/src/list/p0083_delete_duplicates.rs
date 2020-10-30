use crate::{ListNode, Solution};

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut point = head.as_mut();
        while let Some(node) = point {
            if node.next.is_some() && node.next.as_ref().unwrap().val == node.val {
                node.next = node.next.as_mut().unwrap().next.take();
                point = Some(node);
            } else {
                point = node.next.as_mut();
            }
        }
        head
    }
}
