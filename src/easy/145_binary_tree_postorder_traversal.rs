// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;

use crate::easy::Solution;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let r = match root {
            Some(r) => r,
            None => return Vec::new(),
        };
        let mut result = Vec::new();
        let mut queue: Vec<Rc<RefCell<TreeNode>>> = vec![r];
        
        while let Some(val) = queue.pop() {
            let b_val = val.borrow();
            result.push(b_val.val);

            if let Some(left) = b_val.left.clone() {
                queue.push(left);
            }

            if let Some(right) = b_val.right.clone() {
                queue.push(right);
            }
        }
            
        result.into_iter().rev().collect()
    }
}

// 145. Binary Tree Postorder Traversal
// Solved
// Easy
// Topics
// premium lock icon
// Companies
// Given the root of a binary tree, return the postorder traversal of its nodes' values.

// Example 1:
// Input: root = [1,null,2,3]
// Output: [3,2,1]
// Explanation:

// Example 2:
// Input: root = [1,2,3,4,5,null,8,null,null,6,7,9]
// Output: [4,6,7,5,2,9,8,3,1]
// Explanation:

// Example 3:
// Input: root = []
// Output: []
// Example 4:
// Input: root = [1]
// Output: [1]

// Constraints:
// The number of the nodes in the tree is in the range [0, 100].
// -100 <= Node.val <= 100
 