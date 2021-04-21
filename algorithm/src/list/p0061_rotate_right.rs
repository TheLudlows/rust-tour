use crate::{ListNode, Solution};

// todo
impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, _k: i32) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut cur = head.as_mut();

        while let Some(_) = cur {
            cur = cur.unwrap().next.as_mut();
            len += 1;
        }
        None
    }
}
