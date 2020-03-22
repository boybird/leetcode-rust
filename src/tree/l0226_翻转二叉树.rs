/*
 * @lc app=leetcode.cn id=226 lang=rust
 *
 * [226] 翻转二叉树
 */
struct Solution;
#[derive(Debug, PartialEq, Eq)]
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
// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
type Tree = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn invert_tree(root: Tree) -> Tree {
        if let Some(node) = root.clone() {
            Solution::invert_tree(node.borrow().right.clone());
            Solution::invert_tree(node.borrow().left.clone());
            let right = node.borrow().right.clone();
            let left = node.borrow().left.clone();
            node.borrow_mut().left = right;
            node.borrow_mut().right = left;
        }
        root
    }
}
// @lc code=end

mod tests {

    #[test]
    fn test_it() {
        use super::*;
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        dbg!(Solution::invert_tree(tree));
    }
}
