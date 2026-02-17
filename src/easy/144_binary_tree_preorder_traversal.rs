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
   pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        
        if let Some(r) = root {
            stack.push(r);
        }

        while let Some(node_rc) = stack.pop() {
            let node = node_rc.borrow();
            result.push(node.val);
            if let Some(right) = &node.right {
                stack.push(Rc::clone(right));
            }
            if let Some(left) = &node.left {
                stack.push(Rc::clone(left));
            }
        }

        result
    }
}

// 144. Binary Tree Preorder Traversal
// Solved
// Easy
// Topics
// premium lock icon
// Companies
// Given the root of a binary tree, return the preorder traversal of its nodes' values.

// Example 1:
// Input: root = [1,null,2,3]
// Output: [1,2,3]
// Explanation:

// Example 2:
// Input: root = [1,2,3,4,5,null,8,null,null,6,7,9]
// Output: [1,2,4,5,6,7,3,8,9]
// Explanation:

// Example 3:
// Input: root = []
// Output: []
// Example 4:
// Input: root = [1]
// Output: [1]

// Constraints:
// The number of nodes in the tree is in the range [0, 100].
// -100 <= Node.val <= 100
 