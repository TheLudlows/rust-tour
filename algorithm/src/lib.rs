#![feature(type_name_of_val)]

use std::cell::RefCell;
use std::rc::Rc;
use std::borrow::BorrowMut;

pub mod sort;
pub mod math;
pub mod datastruct;
mod array;
mod dp;
mod string;
mod list;
mod heap;
mod search;
mod backtracking;
mod tree;
mod hash;
mod stack;
mod queue;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub(crate) fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
    pub(crate) fn new_withNext(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode {
            next,
            val,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn as_list(arr:&Vec<i32>) -> Option<Box<ListNode>> {
    let mut head =Some(Box::new(ListNode::new(0)));
    let mut curr = head.as_mut();
    for i in arr {
        curr.unwrap().next = Some(Box::new(ListNode::new(*i)));
        curr = curr.unwrap().next.as_mut();
    }
    head.unwrap().next.take()
}

pub struct Solution;
