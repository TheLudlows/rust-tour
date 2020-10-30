use crate::{ListNode, Solution};

impl Solution {
    pub fn reverse_print(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut res = vec![];
        while let Some(n) = head {
            res.push(n.val);
            head = n.next;
        }
        res.reverse();
        res
    }
}
