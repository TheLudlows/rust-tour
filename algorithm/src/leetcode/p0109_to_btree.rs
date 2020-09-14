use std::rc::Rc;
use std::cell::RefCell;
use crate::leetcode::common::{ListNode, TreeNode, Solution};

/// 链表从中间切开,那么这时候主体是一个相对平衡的二叉树
/// 左边,右边分别像第一步一样递归求解即可.
/// Cell和RefCell的区别


impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        if head.is_none() {
            return None;
        }
        let mut cur = &head;
        let mut v = Vec::new();
        while let Some(node) = cur {
            cur = &node.next;
            v.push(node.val);
        }
        gen_tree(&v)
    }
}

fn gen_tree(v: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if v.len() >= 1 {
        let mid = v.len() / 2;
        Some(Rc::new(RefCell::new(TreeNode {
            val: v[mid],
            left: gen_tree(&v[..mid]),
            right: gen_tree(&v[mid + 1..]),
        })))
    } else {
        None
    }
}

#[test]
fn test() {
    let head = Some(Box::new(ListNode::new(1)));
    Solution::sorted_list_to_bst(head);
}