use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};
use std::option::Option::Some;

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                1 + Solution::count_nodes(node.borrow().left.clone())
                    + Solution::count_nodes(node.as_ref().borrow().right.clone())
            }
            None => 0,
        }
    }

    pub fn count_nodes2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(root) => {
                let left = left_depth(root.borrow().left.clone());
                let right = left_depth(root.borrow().right.clone());
                return if left == right {
                    (1<<left) + Self::count_nodes2(root.borrow().right.clone())
                } else {
                    (1<<right) + Self::count_nodes2(root.borrow().left.clone())
                }
            }
            None => 0
        }
    }
}
fn left_depth(mut node:Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut level = 0;

    while let Some (v) = node{
        level+=1;
        node = v.borrow().left.clone();
    }
    level
}


#[test]
pub fn test() {}
