use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};

impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut ret = vec![];
        let mut cur = vec![];
        dfs(&root, &mut cur, &mut ret);
        ret
    }
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, cur: &mut Vec<String>, ret: &mut Vec<String>) {
    if let Some(node_ref ) = node{
        let node_ref = node_ref.borrow();
        cur.push(node_ref.val.to_string());
        if node_ref.left.is_none() && node_ref.right.is_none() {
            ret.push(cur.clone().join("->"));
        } else {
            dfs(&node_ref.left, cur, ret);
            dfs(&node_ref.right, cur, ret);
        }
        cur.pop();
    }
}
