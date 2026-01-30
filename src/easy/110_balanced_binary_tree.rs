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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        let root: Rc<RefCell<TreeNode>> = root.unwrap();
        let borrow_root = root.borrow();

        if borrow_root.left.is_none() && borrow_root.right.is_none() {
            return true;
        }

        Self::bfs(borrow_root.left.clone()) && Self::bfs(borrow_root.right.clone())
    }

    fn bfs(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root = match root {
            Some(r) => r,
            None => return true,
        };

        let mut deque: VecDeque<(Rc<RefCell<TreeNode>>, i32)> = VecDeque::new();

        deque.push_back((root, 0));
        while let Some((node, depth)) = deque.pop_front() {
            match (node.borrow().right.clone(), node.borrow().left.clone()) {
                (Some(l), Some(r)) => {
                    deque.push_back((r, 0));
                    deque.push_back((l, 0));
                }
                (Some(l), None) => {
                    deque.push_back((l, depth + 1));
                    if depth + 1 == 2 {
                        return false;
                    }
                }
                (None, Some(r)) => {
                    deque.push_back((r, depth + 1));
                    if depth + 1 == 2 {
                        return false;
                    }
                }
                (None, None) => continue,
            }
        }

        true
    }
}

// 110. Balanced Binary Tree
// Easy
// Topics
// premium lock icon
// Companies
// Given a binary tree, determine if it is height-balanced.

// Example 1:

// Input: root = [3,9,20,null,null,15,7]
// Output: true
// Example 2:

// Input: root = [1,2,2,3,3,null,null,4,4]
// Output: false
// Example 3:

// Input: root = []
// Output: true

// Constraints:

// The number of nodes in the tree is in the range [0, 5000].
// -104 <= Node.val <= 104
