use crate::leetcode::common::{ListNode, Solution};
/// emm...

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut p, mut q, mut carry) = (l1, l2, 0);
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut current = head.as_mut();
        while p.is_some() || q.is_some() {
            let mut sum = carry;
            if let Some(v) = p {
                sum += v.val;
                p = v.next;
            }
            if let Some(v) = q {
                sum += v.val;
                q = v.next;
            }
            carry = sum / 10;
            let v = current.unwrap();
            v.next = Some(Box::new(ListNode::new(sum % 10)));
            current = v.next.as_mut();
        }
        if carry > 0 {
            let v = current.unwrap();
            v.next = Some(Box::new(ListNode::new(carry)));
        }
        head.unwrap().next
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode::p0002_add_two_num::{build_list};
    use crate::leetcode::common::Solution;

    #[test]
    fn two_sum_test() {
        let a = vec![2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 9];
        let b = vec![5, 6, 4, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 9, 9, 9, 9];
        let l1 = build_list(&a);
        let l2 = build_list(&b);

        Solution::add_two_numbers(l1, l2);
    }
}

fn build_list(arr: &[i32]) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut cur = &mut head;
    for a in arr {
        let node = Some(Box::new(ListNode::new(*a)));
        cur.as_mut().unwrap().next = node;
        cur = &mut cur.as_mut().unwrap().next;
    }
    return head.unwrap().next;
}