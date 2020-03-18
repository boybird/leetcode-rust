/*
 * @lc app=leetcode.cn id=21 lang=rust
 *
 * [21] 合并两个有序链表
 */
struct Solution {}
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Link,
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
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Link {
        Self::merge_two_lists_ref(&l1, &l2)
    }
    pub fn merge_two_lists_ref(list1: &Link, list2: &Link) -> Link {
        if list1.is_none() && list2.is_none() {
            return None;
        }
        let mut res = None;
        {
            let mut node3 = &mut res;
            let mut node1 = list1;
            let mut node2 = list2;
            loop {
                if let (Some(link1), Some(link2)) = (node1, node2) {
                    if link1.val > link2.val {
                        *node3 = Some(Box::new(ListNode::new(link2.val)));
                        node2 = &link2.next;
                    } else {
                        *node3 = Some(Box::new(ListNode::new(link1.val)));
                        node1 = &link1.next;
                    }
                    if let Some(link) = { node3 } {
                        node3 = &mut link.next;
                    } else {
                        panic!();
                    }
                } else if let Some(link1) = node1 {
                    *node3 = Some(Box::new(ListNode::new(link1.val)));
                    node1 = &link1.next;
                    if let Some(link) = { node3 } {
                        node3 = &mut link.next;
                    } else {
                        panic!();
                    }
                } else if let Some(link2) = node2 {
                    *node3 = Some(Box::new(ListNode::new(link2.val)));
                    node2 = &link2.next;
                    if let Some(link) = { node3 } {
                        node3 = &mut link.next;
                    } else {
                        panic!();
                    }
                } else {
                    break;
                }
            }
        }
        res
    }
}
// @lc code=end

mod tests {
    #[test]
    fn testit() {
        use super::*;
        let l1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(4))),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode::new(4))),
            })),
        }));
        dbg!(Solution::merge_two_lists(l1, l2));

        dbg!(Solution::merge_two_lists(
            None,
            Some(Box::new(ListNode::new(0)))
        ));
    }
}
