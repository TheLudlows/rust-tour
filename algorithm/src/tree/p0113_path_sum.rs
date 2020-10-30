use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};

/// VecDeque转Vec 深度优先
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut cur = Vec::new();

        addNode(&root, &mut result, &mut cur, sum);
        result
    }
}

pub fn addNode(
    node: &Option<Rc<RefCell<TreeNode>>>,
    result: &mut Vec<Vec<i32>>,
    cur: &mut Vec<i32>,
    sum: i32,
) {
    match node {
        Some(n) => {
            cur.push(n.borrow().val);
            if None == n.borrow().left && None == n.borrow().right {
                if cur.iter().sum::<i32>() == sum {
                    result.push(cur.clone());
                }
            }
            addNode(&n.borrow().left, result, cur, sum);
            cur.pop();
            addNode(&n.borrow().right, result, cur, sum);
            cur.pop();
        }
        None => cur.push(0),
    }
}

#[test]
fn test() {}
