use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;
use crate::{Solution, TreeNode};

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            0
        }else {
            dep(root.as_ref())
        }
    }
}

fn dep(node: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(n) = node {
        1 + max(dep(n.borrow().left.as_ref()), dep(n.borrow().right.as_ref()))
    } else {
        0
    }
}