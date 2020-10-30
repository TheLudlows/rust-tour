use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = vec![];
        sum(&root, &mut res, 0);
        res.iter().sum()
    }
}

fn sum(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>, mut cur: i32) {
    if let Some(node) = root {
        cur = cur * 10 + node.borrow().val;
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            res.push(cur);
        } else {
            sum(&node.borrow().left, res, cur);
            sum(&node.borrow().right, res, cur);
        }
    }
}

#[test]
fn test() {}
