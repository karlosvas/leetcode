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
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::easy::Solution;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, i32)> = VecDeque::new();
        queue.push_back((root.unwrap(), 1));

        let mut sol: i32 = 0;

        while let Some((val, depth)) = queue.pop_front() {
            sol = sol.max(depth);

            let v_borrowed = val.borrow();
            if let Some(l) = &v_borrowed.left {
                queue.push_back((l.clone(), depth + 1));
            }
            if let Some(r) = &v_borrowed.right {
                queue.push_back((r.clone(), depth + 1));
            }
        }

        sol
    }
}

// 104. Maximum Depth of Binary Tree
// Solved
// Easy
// Topics
// premium lock icon
// Companies
// Given the root of a binary tree, return its maximum depth.

// A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.

// Example 1:

// Input: root = [3,9,20,null,null,15,7]
// Output: 3
// Example 2:

// Input: root = [1,null,2]
// Output: 2

// Constraints:

// The number of nodes in the tree is in the range [0, 104].
// -100 <= Node.val <= 100
