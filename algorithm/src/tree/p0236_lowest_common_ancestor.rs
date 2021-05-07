use std::cell::RefCell;
use std::rc::Rc;
use crate::{Solution, TreeNode};

/// 二叉树的最近公共祖先

impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        find(&root, p.unwrap().borrow().val, q.unwrap().borrow().val)
    }
}

fn find(root: &Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(node) => {
            if node.borrow().val == p || node.borrow().val == q {
                return root.clone();
            }
            let left = find(&node.borrow().left, p, q);
            let right = find(&node.borrow().right, p, q);
            if left == None {
                right
            } else if right == None {
                left
            } else {
                root.clone()
            }
        }
        None => None
    }
}

#[test]
fn test() {}