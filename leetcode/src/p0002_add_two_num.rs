pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let l1_sum = get_sum(&l1);
    let l2_sum = get_sum(&l2);
    let mut sum = l1_sum + l2_sum;
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut root = &mut head;
    if sum == 0 {
        return head;
    }
    while sum != 0 {
        let node = Some(Box::new(ListNode::new(sum.wrapping_rem(10) as i32)));
        root.as_mut().unwrap().next = node;
        root = &mut root.as_mut().unwrap().next;
        sum = sum / 10;
    }
    head.unwrap().next
}

fn get_sum(head: &Option<Box<ListNode>>) -> i128 {
    let mut root = head;
    let mut sum: i128 = 0;
    let mut bit: i128 = 1;
    while let Some(node) = root {
        sum += node.val as i128 * bit;
        bit *= 10;
        root = &node.next;
    }
    println!("{}",sum);
    sum
}

#[cfg(test)]
mod test {
    use crate::p0002_add_two_num::{ListNode, build_list, add_two_numbers};

    #[test]
    fn two_sum_test() {
        let a = vec![2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 9];
        let b = vec![5, 6, 4, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 9, 9, 9, 9];
        let l1 = build_list(&a);
        let l2 = build_list(&b);
        add_two_numbers(l1,l2);
    }
}
fn build_list(arr : &[i32]) -> Option<Box<ListNode>>{
    let mut head =  Some(Box::new(ListNode::new(0)));
    let mut cur = &mut head;
    for a in arr{
        let node = Some(Box::new(ListNode::new(*a)));
        cur.as_mut().unwrap().next = node;
        cur = &mut cur.as_mut().unwrap().next;
    }
    return head.unwrap().next;
}