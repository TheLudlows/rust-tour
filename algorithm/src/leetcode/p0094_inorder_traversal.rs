use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::leetcode::common::{Solution, TreeNode};

impl Solution {
    /// 递归遍历
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = Vec::new();
        traversal(root.as_ref(), &mut v);
        println!("{:?}", root);
        v
    }
    /// 迭代
    pub fn inorder_traversal_iter(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack = VecDeque::new();
        stack.push_back(root.as_ref());
        while Some(node) = stack.pop_back() {}
        vec![]
    }
}

fn traversal(node: Option<&Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
    if let Some(n) = node {
        traversal(n.borrow().left.as_ref(), vec);
        vec.push(n.borrow().val);
        traversal(n.borrow().right.as_ref(), vec);
    }
}


#[test]
fn test() {}