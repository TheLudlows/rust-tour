use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;


use crate::{Solution, TreeNode};

/// 层级遍历 队列
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let result = vec![];

        result
    }
}