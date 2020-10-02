use crate::math::common::{ListNode, Solution};
use crate::{Solution, ListNode};

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut remain = head;
        let mut result = ListNode::new(0);
        let mut tail = &mut result;
        while let Some(mut n1) = remain {
            remain = n1.next.take(); // take()将n1打断，这样n1只有一个值，返回值是除n1节点外的剩余节点
            if let Some(mut n2) = remain {
                remain = n2.next.take(); // take()将n2打断，n2只有一个值，返回值是除n2节点外的剩余节点
                n2.next = Some(n1);
                tail.next = Some(n2);
                tail = tail.next.as_mut().unwrap().next.as_mut().unwrap();//更新tail指针
            } else {
                tail.next = Some(n1);
                tail = tail.next.as_mut().unwrap();
            }
        }
        result.next
    }
}

#[inline]
unsafe fn copy<T>(x: *mut T, y: *mut T) {
    std::ptr::copy_nonoverlapping(x, y, 1)
}
