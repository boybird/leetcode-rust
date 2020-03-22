/*
 * @lc app=leetcode.cn id=102 lang=rust
 *
 * [102] 二叉树的层次遍历
 */
struct Solution;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
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
// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

use std::cell::RefCell;
use std::rc::Rc;
type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn level_order(root: Tree) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut q = std::collections::VecDeque::new();
        if root.is_none() {
            return result;
        }

        let a = root.unwrap().clone();
        q.push_back(a);
        loop {
            if q.is_empty() {
                break;
            }
            let s = q.len();
            let mut layer: Vec<i32> = vec![];
            for _ in 0..s {
                let cur = q.pop_front().unwrap();
                layer.push(cur.clone().borrow().val);
                match cur.clone().borrow().left {
                    Some(ref c) => {
                        q.push_back(c.clone());
                    }
                    _ => {}
                }
                match cur.clone().borrow().right {
                    Some(ref c) => {
                        q.push_back(c.clone());
                    }
                    _ => {}
                }
            }
            result.push(layer)
        }

        result
    }
}
// @lc code=end

