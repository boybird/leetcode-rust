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

// your code below
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(0));
        let (mut p, mut q, mut carry) = (l1, l2, 0);
        let mut x: i32;
        let mut y: i32;
        {
            let mut current = head.as_mut();
            while p.is_some() || q.is_some() {
                if let Some(v) = p {
                    x = v.val;
                    p = v.next;
                } else {
                    x = 0;
                }
                if let Some(v) = q {
                    y = v.val;
                    q = v.next;
                } else {
                    y = 0;
                }

                let sum = x + y + carry;
                carry = sum / 10;

                current.next = Some(Box::new(ListNode::new(sum % 10)));
                unsafe {
                    current = (*(*(current as *mut ListNode)).next.as_mut().unwrap()).as_mut();
                }
            }
            if carry > 0 {
                current.next = Some(Box::new(ListNode::new(1)));
            }
        }
        head.next
    }
}
