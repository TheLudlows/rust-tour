use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};

/// 二叉搜索树的最近公共祖先
///
impl Solution {
    pub fn lowest_common_ancestor1(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        lowest(&root, p.unwrap().borrow().val, q.unwrap().borrow().val)
    }
}

fn lowest(root: &Option<Rc<RefCell<TreeNode>>>, left: i32, right: i32) -> Option<Rc<RefCell<TreeNode>>> {
    return match root {
        Some(node) => {
            let v = node.borrow().val;
            if v > left && v > right {
                lowest(&node.borrow().left, left, right)
            } else if v < left && v < right {
                lowest(&node.borrow().right, left, right)
            } else {
                root.clone()
            }
        }
        None => {
            None
        }
    };
}