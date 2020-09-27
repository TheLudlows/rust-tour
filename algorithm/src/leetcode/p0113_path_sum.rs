use std::rc::Rc;
use std::cell::RefCell;
use crate::leetcode::common::{Solution, TreeNode};
use std::collections::VecDeque;
/// VecDeque转Vec
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut cur  =  VecDeque::new();

        addNode(&root, &mut result, &mut cur, sum);
        result
    }

}
pub fn addNode(node:&Option<Rc<RefCell<TreeNode>>>, result:&mut Vec<Vec<i32>>, cur:&mut VecDeque<i32>,sum:i32) {

    match node {
        Some(n) => {
            cur.push_back(n.borrow().val);
            if  None == n.borrow().left && None == n.borrow().right {
                if cur.iter().sum::<i32>() == sum {
                    result.push(Vec::from(cur.as_slices().0));
                }
            }
            addNode(&n.borrow().left,result, cur,sum);
            cur.pop_back();
            addNode(&n.borrow().right,result,cur,sum);
            cur.pop_back();
        },
        None => {
            cur.push_back(0)
        }
    }
}
#[test]
fn test() {

}