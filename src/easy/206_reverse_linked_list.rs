// Problem: Reverse Linked List
// Difficulty: Easy

#[allow(dead_code)]
pub struct Solution;
#[derive(PartialEq, Eq, Clone, Debug)]

pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

crate::impl_list_like!(ListNode);
crate::impl_list_new!(ListNode);

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head;
        let mut acc: Option<Box<ListNode>> = None;

        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = acc;
            acc = Some(node);
            current = next;
        }

        acc
    }
}

// Given the `head` of a singly linked list, reverse the list, and return _the reversed list_.
//
// Example 1:
// Input: head = [1,2,3,4,5]
// Output: [5,4,3,2,1]
//
// Example 2:
// Input: head = [1,2]
// Output: [2,1]
//
// Example 3:
// Input: head = []
// Output: []
//
// Constraints:
// - The number of nodes in the list is the range `[0, 5000]`.
// - `-5000 <= Node.val <= 5000`
//
// Follow up: A linked list can be reversed either iteratively or recursively. Could you implement both?

#[cfg(test)]
mod tests {
    use super::*;

    crate::check_case!(
        c1,
        crate::list_to_vec(&Solution::reverse_list(crate::create_list::<ListNode>(&[1, 2, 3, 4, 5]))),
        [5, 4, 3, 2, 1]
    );

    crate::check_case!(
        c2,
        crate::list_to_vec(&Solution::reverse_list(crate::create_list::<ListNode>(&[1, 2]))),
        [2, 1]
    );

    crate::check_case!(
        c3,
        crate::list_to_vec(&Solution::reverse_list(crate::create_list::<ListNode>(&[]))),
        []
    );
}
