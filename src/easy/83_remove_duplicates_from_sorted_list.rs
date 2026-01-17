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
use std::vec::Vec;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy: Option<Box<_>> = Some(Box::new(ListNode::new(0)));
        let mut tail: &mut Option = &mut dummy;
        let mut current: &Option = &head;

        let mut prev_val: Option = None;

        while let Some(node) = current {
            if node.next.is_none() || node.next.as_ref().unwrap().val != node.val {
                if prev_val.is_none() || prev_val.unwrap() != node.val {
                    if let Some(t) = tail {
                        t.next = Some(Box::new(ListNode::new(node.val)));
                        tail = &mut t.next;
                    }
                }
                prev_val = Some(node.val);
            }
            current = &node.next;
        }

        dummy.and_then(|d| d.next)
    }
}
