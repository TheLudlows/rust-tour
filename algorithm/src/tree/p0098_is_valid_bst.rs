use std::cell::RefCell;
use std::rc::Rc;

use crate::math::common::{Solution, TreeNode};
use crate::{TreeNode, Solution};

/// 二叉搜索树 只需要左子树的节点小于当前根节点，有子树大于根节点即可
///
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        check(root, None, None)
    }
}

pub fn check(mid: Option<Rc<RefCell<TreeNode>>>, min: Option<i32>, max: Option<i32>) -> bool {
    match mid {
        Some(node) => {
            let mid_v = node.borrow().val;
            min.map_or(true, |x| x < mid_v)
                && max.map_or(true, |x| x > mid_v)
                && check(node.borrow().left.clone(), min, Some(mid_v))
                && check(node.borrow().right.clone(), Some(mid_v), max)
        }
        None => {
            true
        }
    }
}

#[test]
fn test() {}