// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
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

struct Solution {}
impl Clone for ListNode {
    fn clone(&self) -> ListNode {
        ListNode {
            val: self.val,
            next: match &self.next {
                None => None,
                Some(b) => Some(b.clone()),
            },
        }
    }
}
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut curs = lists
            .iter()
            .map(|_o| Box::new(ListNode::new(0)))
            .collect::<Vec<Box<ListNode>>>();
        while !curs.iter().any(|c| c.as_mut().is_none()) {}
        None
    }
}
