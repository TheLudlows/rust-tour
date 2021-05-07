use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};

///  深度优先 先序遍历
impl Solution {
    pub fn path_sum_(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut cur = Vec::new();
        add_node(&root, &mut result, &mut cur, sum);
        result
    }
}

pub fn add_node(
    node: &Option<Rc<RefCell<TreeNode>>>,
    result: &mut Vec<Vec<i32>>,
    cur: &mut Vec<i32>,
    sum: i32,
) {
    if let Some(n) = node {
        cur.push(n.borrow().val);
        if None == n.borrow().left && None == n.borrow().right {
            if cur.iter().sum::<i32>() == sum {
                result.push(cur.clone());
            }
        }
        add_node(&n.borrow().left, result, cur, sum);
        add_node(&n.borrow().right, result, cur, sum);
        cur.pop();
    }
}
