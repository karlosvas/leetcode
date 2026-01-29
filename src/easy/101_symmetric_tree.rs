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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue = VecDeque::new();

        let root = root.unwrap();
        queue.push_back((root.borrow().left.clone(), root.borrow().right.clone()));

        while let Some((left, rigth)) = queue.pop_front() {
            match (left, rigth) {
                (Some(l), Some(r)) => {
                    let l_borrow = l.borrow();
                    let r_borrow = r.borrow();

                    if l_borrow.val != r_borrow.val {
                        return false;
                    }

                    queue.push_back((l_borrow.left.clone(), r_borrow.right.clone()));
                    queue.push_back((l_borrow.right.clone(), r_borrow.left.clone()));
                }
                (None, None) => continue,
                _ => return false,
            };
        }

        true
    }
}

// 101. Symmetric Tree
// Solved
// Easy
// Topics
// premium lock icon
// Companies
// Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).

// Example 1:

// Input: root = [1,2,2,3,4,4,3]
// Output: true
// Example 2:

// Input: root = [1,2,2,null,3,null,3]
// Output: false

// Constraints:

// The number of nodes in the tree is in the range [1, 1000].
// -100 <= Node.val <= 100
