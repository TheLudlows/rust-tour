use crate::math::common::{ListNode, Solution};
use crate::{Solution, ListNode};

///
/// take的用法
impl Solution {
    pub fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l3 = ListNode::new(0);
        // cur 指向list3的最新的节点
        let mut cur = &mut l3;
        while let (Some(n1), Some(n2)) = (l1.as_ref(), l2.as_ref()) {
            if n1.val < n2.val {
                //将较小链表连接到新链表尾节点，所有权移动
                cur.next = l1;
                //将cur指向它的后继节点
                cur = cur.next.as_mut().unwrap();
                //将链表从尾节点取下来，将所有权返给较小的链表
                l1 = cur.next.take();
            } else {
                cur.next = l2;
                cur = cur.next.as_mut().unwrap();
                l2 = cur.next.take();
            }
        }
        //剩余部分所有权移动至list3中
        cur.next = if l1.is_some() { l1 } else { l2 };
        l3.next
    }
}