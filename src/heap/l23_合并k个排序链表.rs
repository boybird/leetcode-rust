/*
 * @lc app=leetcode.cn id=23 lang=rust
 *
 * [23] 合并K个排序链表
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
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let len = lists.len();
        if len == 0 {
            return None;
        }
        if len == 1 {
            return lists[0].clone();
        }
        if len == 2 {
            let (l1, l2) = (&lists[0], &lists[1]);
            return Self::merge2(l1, l2);
        }
        //lists.chunks(2)
        let _i: Vec<_> = lists
            .chunks(2)
            //.map(|p| Self::merge_k_lists(dbg!(p.to_vec())))
            .map(|p| Self::merge_k_lists(p.to_vec()))
            .collect();
        Self::merge_k_lists(_i)
    }

    pub fn merge2(
        mut list1: &Option<Box<ListNode>>,
        mut list2: &Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut res = &mut result;

        let result = loop {
            if list1.is_none() && list2.is_none() {
                break result;
            }
            if list1.is_none() {
                while let Some(n) = list2 {
                    list2 = &n.next;
                    let next_node = ListNode::new(n.val);
                    *res = Some(Box::new(next_node));
                    if let Some(node) = res {
                        res = &mut node.next;
                    }
                }
            }
            if list2.is_none() {
                while let Some(n) = list1 {
                    list1 = &n.next;
                    let next_node = ListNode::new(n.val);
                    *res = Some(Box::new(next_node));
                    if let Some(node) = res {
                        res = &mut node.next;
                    }
                }
            }
            *res = list1.as_ref().and_then(|_p1| {
                list2.as_ref().and_then(|_p2| {
                    if _p1.val > _p2.val {
                        list2 = &_p2.next;
                        Some(Box::new(ListNode::new(_p2.val)))
                    } else {
                        list1 = &_p1.next;
                        Some(Box::new(ListNode::new(_p1.val)))
                    }
                })
            });
            if let Some(node) = res {
                res = &mut node.next;
            }
        };
        //println!("{:?}", result);
        result
    }
}
// @lc code=end
mod tests {
    #[test]
    fn testit() {
        use super::*;
        let (mut l01, mut l02, l03) = (ListNode::new(1), ListNode::new(4), ListNode::new(5));
        l02.next = Some(Box::new(l03));
        l01.next = Some(Box::new(l02));
        let l0 = Some(Box::new(l01));

        let (mut l01, mut l02, l03) = (ListNode::new(1), ListNode::new(3), ListNode::new(4));
        l02.next = Some(Box::new(l03));
        l01.next = Some(Box::new(l02));
        let l1 = Some(Box::new(l01));

        let (mut l01, l02) = (ListNode::new(2), ListNode::new(6));
        l01.next = Some(Box::new(l02));
        let l2 = Some(Box::new(l01));

        let l = vec![l0, l1, l2];
        let p = Solution::merge_k_lists(l);
        println!("{:?}", p);
    }
}
