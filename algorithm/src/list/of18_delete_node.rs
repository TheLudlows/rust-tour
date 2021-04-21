use crate::{ListNode, Solution};

impl Solution {
    pub fn delete_node(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        if head.as_ref().unwrap().val == val {
            return head.unwrap().next;
        }
        let mut cur = head.as_mut();
        while let Some(node) = cur {
            match node.next.as_mut() {
                None => break,
                Some(next) => {
                    if next.val == val {
                        node.next = next.next.take();
                        break;
                    }
                }
            }
            cur = node.next.as_mut();
        }
        head
    }
}
