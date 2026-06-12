use crate::medium::Solution;

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

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut current_l1, mut current_l2) = (l1, l2);
        let mut final_sol: Option<Box<ListNode>> = None;
        let mut current_sol: &mut Option<Box<ListNode>> = &mut final_sol;
        let mut carry: i32 = 0;

        while current_l1.is_some() || current_l2.is_some() {
            let mut sum = 0;

            if let Some(i) = current_l1 {
                sum += i.val;
                current_l1 = i.next;
            }

            if let Some(j) = current_l2 {
                sum += j.val;
                current_l2 = j.next;
            }

            sum += carry;
            carry = if sum > 9 {
                sum -= 10;
                1
            } else {
                0
            };

            let new_node = Some(Box::new(ListNode::new(sum)));
            *current_sol = new_node;
            current_sol = &mut current_sol.as_mut().unwrap().next;
        }

        if carry == 1 {
            let new_node = Some(Box::new(ListNode::new(1)));
            *current_sol = new_node;
            current_sol = &mut current_sol.as_mut().unwrap().next;
        }

        final_sol
    }
}

// 2. Add Two Numbers
// Solved
// Medium
// Topics
// premium lock iconCompanies
//
// You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
//
// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
//
//
//
// Example 1:
//
// Input: l1 = [2,4,3], l2 = [5,6,4]
// Output: [7,0,8]
// Explanation: 342 + 465 = 807.
//
// Example 2:
//
// Input: l1 = [0], l2 = [0]
// Output: [0]
//
// Example 3:
//
// Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
// Output: [8,9,9,9,0,0,0,1]
//
//
//
// Constraints:
//
//     The number of nodes in each linked list is in the range [1, 100].
//     0 <= Node.val <= 9
//     It is guaranteed that the list represents a number that does not have leading zeros.
//
