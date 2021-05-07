use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};

/// 二叉搜索树 只需要左子树的节点小于当前根节点，右子树大于根节点即可
///
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        check(root, i64::MIN, i64::MAX)
    }
}

pub fn check(mid: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
    match mid {
        Some(node) => {
            let val = node.borrow().val as i64;
            val > min && max > val
                && check(node.borrow().left.clone(), min, val)
                && check(node.borrow().right.clone(), val, max)
        }
        None => true,
    }
}
