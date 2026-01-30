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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let root: Rc<RefCell<TreeNode>> = match root {
            Some(r) => r,
            None => return 0,
        };

        let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, i32)> = VecDeque::new();

        queue.push_back((root, 1));

        while let Some((node, depth)) = queue.pop_front() {
            let n = node.borrow();

            if n.left.is_none() && n.right.is_none() {
                return depth;
            }

            if let Some(left) = &n.left {
                queue.push_back((left.clone(), depth + 1));
            }
            if let Some(right) = &n.right {
                queue.push_back((right.clone(), depth + 1));
            }
        }

        0
    }
}

// 111. Minimum Depth of Binary Tree
// Easy
// Topics
// premium lock icon
// Companies
// Given a binary tree, find its minimum depth.

// The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.

// Note: A leaf is a node with no children.

// Example 1:

// Input: root = [3,9,20,null,null,15,7]
// Output: 2
// Example 2:

// Input: root = [2,null,3,null,4,null,5,null,6]
// Output: 5

// Constraints:

// The number of nodes in the tree is in the range [0, 105].
// -1000 <= Node.val <= 1000
