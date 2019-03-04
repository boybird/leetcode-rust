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
        let mut skip = true;
        let mut r = None;
        let mut cur = None;
        let mut pointers = lists.clone();
        while skip {
            // pointers.iter().enumerate()
            for (i, li) in pointers.iter().enumerate() {
                if li.is_some() {
                    println!("{:?}", li);
                    if r.is_none() {
                        r = Some(Box::new(ListNode::new(li.clone().unwrap().val)));
                        cur = r.clone();
                    }
                }
            }
        }
        r
    }
}
