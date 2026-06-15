// Problem: Remove Linked List Elements
// Difficulty: Easy

pub struct Solution;

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        while head.as_ref().is_some_and(|n| n.val == val) {
            head = head.unwrap().next;
        }

        let mut current = head.as_mut();
        while let Some(node) = current {
            while node.next.as_ref().is_some_and(|n| n.val == val) {
                node.next = node.next.take().unwrap().next;
            }

            current = node.next.as_mut();
        }

        head
    }
}

// Given the `head` of a linked list and an integer `val`,
// remove all the nodes of the linked list that has `Node.val == val`,
// and return *the new head*.
//
// Example 1:
// Input: head = [1,2,6,3,4,5,6], val = 6
// Output: [1,2,3,4,5]
//
// Example 2:
// Input: head = [], val = 1
// Output: []
//
// Example 3:
// Input: head = [7,7,7,7], val = 7
// Output: []
//
// Constraints:
// - The number of nodes in the list is in the range `[0, 10^4]`.
// - `1 <= Node.val <= 50`
// - `0 <= val <= 50`

crate::impl_list_like!(ListNode);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list_to_string;

    crate::check_case!(
        c1,
        list_to_string(&Solution::remove_elements(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 6,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode {
                                    val: 5,
                                    next: Some(Box::new(ListNode { val: 6, next: None })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
            6
        )),
        "[1,2,3,4,5]" | "1,2,3,4,5"
    );

    crate::check_case!(
        c2,
        list_to_string(&Solution::remove_elements(None, 1)),
        "[]" | ""
    );

    crate::check_case!(
        c3,
        list_to_string(&Solution::remove_elements(
            Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 7,
                    next: Some(Box::new(ListNode {
                        val: 7,
                        next: Some(Box::new(ListNode { val: 7, next: None })),
                    })),
                })),
            })),
            7
        )),
        "[]" | ""
    );
}
