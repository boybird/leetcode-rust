/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] 两数相加
 */
struct Solution;
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
type Link = Option<Box<ListNode>>;
impl Solution {
    pub fn add_two_numbers(l1: Link, l2: Link) -> Link {
        let mut res = None;
        {
            let mut node3 = &mut res;
            let mut node1 = &l1;
            let mut node2 = &l2;
            let mut step = 0;
            let mut v;
            loop {
                if let (Some(link1), Some(link2)) = (node1, node2) {
                    v = link1.val + link2.val + step;
                    if v >= 10 {
                        step = 1;
                        v = v - 10;
                    } else {
                        step = 0;
                    }
                    *node3 = Some(Box::new(ListNode::new(v)));
                    node2 = &link2.next;
                    node1 = &link1.next;
                } else if let Some(link1) = node1 {
                    v = link1.val + step;
                    if v >= 10 {
                        step = 1;
                        v = v - 10;
                    } else {
                        step = 0;
                    }
                    node1 = &link1.next;
                    *node3 = Some(Box::new(ListNode::new(v)));
                } else if let Some(link2) = node2 {
                    v = link2.val + step;
                    if v >= 10 {
                        step = 1;
                        v = v - 10;
                    } else {
                        step = 0;
                    }
                    node2 = &link2.next;
                    *node3 = Some(Box::new(ListNode::new(v)));
                } else {
                    if step > 0 {
                        *node3 = Some(Box::new(ListNode::new(step)));
                        step = 0;
                    } else {
                        break;
                    }
                }
                if let Some(link) = { node3 } {
                    node3 = &mut link.next;
                } else {
                    panic!();
                }
            }
        }
        res
    }
}
// @lc code=end

mod tests {
    #[test]
    fn testit() {}
}
