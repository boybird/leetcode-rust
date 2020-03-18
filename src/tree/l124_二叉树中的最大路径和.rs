/*
 * @lc app=leetcode.cn id=124 lang=rust
 *
 * [124] 二叉树中的最大路径和
 */
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

struct Solution;
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
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = std::i32::MIN;
        if root.is_none() {
            return 0;
        }

        Self::max_gain(root, &mut |x| max = max.max(x));

        max
    }
    pub fn max_gain(node: Option<Rc<RefCell<TreeNode>>>, f: &mut impl FnMut(i32)) -> i32 {
        if node.is_none() {
            return 0;
        }
        let mut r_b = node.as_ref().unwrap().borrow_mut();
        let (l, r) = (r_b.left.take(), r_b.right.take());
        let (lval, rval) = (Self::max_gain(l, f).max(0), Self::max_gain(r, f).max(0));
        // println!("val: {},left: {}, right: {}", r_b.val, lval, rval);
        let cur_path_max = lval + rval + r_b.val;
        f(cur_path_max);
        (lval + r_b.val).max(rval + r_b.val)
    }
}
// @lc code=end

mod tests {

    #[test]
    fn test_it() {
        use super::*;

        let mut t = TreeNode::new(1);
        t.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        t.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

        println!("{:#?}", t);

        let max = Solution::max_path_sum(Some(Rc::new(RefCell::new(t))));
        println!("max: {}", max);

        let mut t = TreeNode::new(-10);
        t.left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let mut t_right = TreeNode::new(20);
        t_right.left = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        t_right.right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        t.right = Some(Rc::new(RefCell::new(t_right)));
        println!("{:#?}", t);

        let max = Solution::max_path_sum(Some(Rc::new(RefCell::new(t))));
        println!("max: {}", max);
    }
}
