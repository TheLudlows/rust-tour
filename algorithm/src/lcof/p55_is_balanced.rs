use std::rc::Rc;
use std::cell::RefCell;
use crate::{Solution, TreeNode};
use std::cmp::max;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        balanced(root.as_ref()) != -1
    }
}


fn balanced(node: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(n) = node {
        let left = balanced(n.borrow().left.as_ref());
        let right = balanced(n.borrow().right.as_ref());
        if left == -1 ||right == -1 {
            return -1;
        }
        let gap = (left - right).abs();
        return if gap < 2 {
            max(left, right) + 1
        } else {
            -1
        }
    } else {
        0
    }
}