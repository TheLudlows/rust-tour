use std::rc::Rc;
use std::cell::RefCell;
use crate::leetcode::common::{Solution, TreeNode};
use std::collections::VecDeque;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let result = vec![];
        while let Some() = queue.pop_front() {

        }
        result
    }
}