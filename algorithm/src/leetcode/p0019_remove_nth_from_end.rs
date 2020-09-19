use crate::leetcode::common::{ListNode, Solution};
/// 注意as_mut 返回 Option<&mut T>
/// as_ref 返回的是  Option<& T>
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        dummy.next = head;
        let mut dummy = Box::new(dummy);
        let mut fast = dummy.clone();
        let mut slow = dummy.as_mut();
        // move fast n forward
        for _ in 0..n {
            fast = fast.next.unwrap();
        }

        while fast.next.is_some() {
            fast = fast.next.unwrap();
            slow = slow.next.as_mut().unwrap();
        }
        let next = slow.next.as_mut().unwrap();
        slow.next = next.next.clone();
        dummy.next
    }
}
# [test]
fn test() {
let mut tail = Some(Box::new(ListNode::new(3)));
let mut mid = Some(Box::new(ListNode::new_withNext(2, tail)));

let mut head = Some(Box::new(ListNode::new_withNext(1, mid)));
Solution::remove_nth_from_end(head, 1);
}