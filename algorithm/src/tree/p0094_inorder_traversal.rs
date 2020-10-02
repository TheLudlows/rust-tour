use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;


use crate::{Solution, TreeNode};

impl Solution {
    /// 递归遍历
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = Vec::new();
        traversal(root.as_ref(), &mut v);
        println!("{:?}", root);
        v
    }
    /// 中序迭代
    pub fn mid_iter(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut result = Vec::new();
        while !stack.is_empty() || root.is_some() {
            if root.is_some() {
                stack.push(root.clone());
                root = root.unwrap().borrow().left.clone();
            } else {
                let node = stack.pop().unwrap().unwrap();
                result.push(node.borrow().val);
                root = node.borrow().right.clone();
            }
        }
        result
    }

    /// 前序迭代
    pub fn preorder_iter(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut answer: Vec<i32> = Vec::new();
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![root];
        while let Some(last) = stack.pop() {
            if let Some(n) = last {
                answer.push(n.borrow().val);
                stack.push(n.borrow().right.clone());
                stack.push(n.borrow().left.clone());
            }
        }
        answer
    }

    /// 后序迭代,前序的方式 逆序
    pub fn posorder_iter(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut answer: Vec<i32> = Vec::new();
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![root];
        while let Some(last) = stack.pop() {
            if let Some(n) = last {
                answer.push(n.borrow().val);
                stack.push(n.borrow().left.clone());
                stack.push(n.borrow().right.clone());
            }
        }
        answer.reverse();
        answer
    }
}

fn traversal(node: Option<&Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
    if let Some(n) = node {
        traversal(n.borrow().left.as_ref(), vec);
        vec.push(n.borrow().val);
        traversal(n.borrow().right.as_ref(), vec);
    }
}


#[test]
fn test() {}