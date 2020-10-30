use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::{Solution, TreeNode};

/// 层级遍历 队列
/// vec和VecDequeue区别
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());
        let mut result = vec![];
        while !queue.is_empty() {
            let n = queue.len();
            let mut level = vec![];
            for _ in 0..n {
                if let Some(Some(node)) = queue.pop_front() {
                    level.push(node.borrow().val);
                    queue.push_back(node.borrow().left.clone());
                    queue.push_back(node.borrow().right.clone());
                }
            }
            if level.len() != 0 {
                result.push(level);
            }
        }
        result
    }
}

#[test]
fn test() {
    let mut v = vec![];
    v.push(1);
    v.push(2);

    println!("{:?}", v.pop());
}
