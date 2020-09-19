use crate::leetcode::common::{ListNode, Solution};

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut fast = Box::new( ListNode::new(0));
        fast.next = head;
        let mut slow = fast.clone();
        //println!("{:p}",slow.next.unwrap());
        //println!("{:p}",fast.next.unwrap());
        for _ in 0..n {
            fast = fast.next.unwrap();
        }
        while fast.next.is_some() {
            fast = fast.next.unwrap();
            slow = slow.next.unwrap();
        }
        return Some(slow.next.unwrap().clone());
    }
}
#[test]
fn test() {
    let mut tail = Some(Box::new(ListNode::new(3)));
    let mut mid = Some(Box::new(ListNode::new_withNext(2,tail)));

    let mut head = Some(Box::new(ListNode::new_withNext(1,mid)));
    Solution::remove_nth_from_end(head,1);
}