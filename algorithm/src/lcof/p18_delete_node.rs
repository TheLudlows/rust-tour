use crate::{ListNode, Solution};

impl Solution {
    pub fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        if head.as_ref().unwrap().val == val {
            return head.unwrap().next;
        }
        let mut head = head;
        let mut node = head.as_mut();
        while let Some(prev) = node {
            match prev.next.as_mut() {
                None => break,
                Some(cur) => {
                    if cur.val == val {
                        prev.next = cur.next.take();
                        break;
                    }
                }
            }
            node = prev.next.as_mut();
        }
        head
    }
}
