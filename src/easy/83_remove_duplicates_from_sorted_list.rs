// Definition for singly-linked list.
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
use crate::easy::Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy: Option<Box<_>> = Some(Box::new(ListNode::new(0)));
        let mut tail: &mut Option<Box<ListNode>> = &mut dummy;
        let mut current: &Option<Box<ListNode>> = &head;

        let mut prev_val: Option<i32> = None;

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

// 83. Remove Duplicates from Sorted List
// Given the head of a sorted linked list, delete all duplicates such that each element appears only once. Return the linked list sorted as well.

// Example 1:

// Input: head = [1,1,2]
// Output: [1,2]
// Example 2:

// Input: head = [1,1,2,3,3]
// Output: [1,2,3]

// Constraints:

// The number of nodes in the list is in the range [0, 300].
// -100 <= Node.val <= 100
// The list is guaranteed to be sorted in ascending order.
