use std::rc::Rc;
use std::cell::RefCell;
use crate::{Solution, TreeNode};

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        build(&preorder[..],&inorder[..]);
    }
}