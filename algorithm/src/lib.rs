#![feature(type_name_of_val)]

use std::cell::RefCell;
use std::rc::Rc;

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

pub struct Solution;
