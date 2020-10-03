use std::cell::RefCell;
use std::cmp::min;
use std::rc::Rc;

use crate::{Solution, TreeNode};
/// 叶子节点的定义
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            0
        }else {
            dep(root.as_ref())
        }
    }
}

fn dep(node: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(n) = node {
        let left = dep(n.borrow().left.as_ref());
        let right = dep(n.borrow().right.as_ref());
        if left == 0 || right == 0 {
            1 + left + right
        } else {
            1 + min(left, right)
        }
    } else {
        0
    }
}