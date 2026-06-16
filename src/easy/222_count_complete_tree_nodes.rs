// 222. Count Complete Tree Nodes
// Easy

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
use std::rc::Rc;

crate::impl_tree_node_new!(TreeNode);

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root) = root else {
            return 0;
        };

        let mut left = root.borrow().left.clone();
        let mut right = root.borrow().right.clone();

        let i_left = left.clone();
        let i_right = right.clone();

        let mut h: i32 = 0;

        loop {
            match (left, right) {
                (Some(l), Some(r)) => {
                    left = l.borrow().left.clone();
                    right = r.borrow().right.clone();
                    h += 1;
                }
                (Some(_), None) | (None, Some(_)) => {
                    return 1 + Self::count_nodes(i_left) + Self::count_nodes(i_right);
                }
                (None, None) => {
                    break;
                }
            };
        }

        2_i32.pow(h as u32 + 1) - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{tree_leaf, tree_node};

    // [1,2,3,4,5,6]
    crate::check_case!(
        c1,
        Solution::count_nodes(tree_node::<TreeNode>(
            1,
            tree_node(2, tree_leaf(4), tree_leaf(5)),
            tree_node(3, tree_leaf(6), None)
        )),
        6
    );

    // []
    crate::check_case!(c2, Solution::count_nodes(None), 0);

    // [1]
    crate::check_case!(c3, Solution::count_nodes(tree_leaf::<TreeNode>(1)), 1);

    // perfect tree of height 2: [1,2,3,4,5,6,7]
    crate::check_case!(
        c4,
        Solution::count_nodes(tree_node::<TreeNode>(
            1,
            tree_node(2, tree_leaf(4), tree_leaf(5)),
            tree_node(3, tree_leaf(6), tree_leaf(7))
        )),
        7
    );
}

// Given the root of a complete binary tree, return the number of the nodes in the tree.
// According to Wikipedia, every level, except possibly the last, is completely filled in a complete binary tree, and all nodes in the last level are as far left as possible. It can have between 1 and 2h nodes inclusive at the last level h.
// Design an algorithm that runs in less than O(n) time complexity.
//
// Example 1:
// Input: root = [1,2,3,4,5,6]
// Output: 6
//
// Example 2:
// Input: root = []
// Output: 0
//
// Example 3:
// Input: root = [1]
// Output: 1
//
// Constraints:
//     The number of nodes in the tree is in the range [0, 5 * 104].
//     0 <= Node.val <= 5 * 104
//     The tree is guaranteed to be complete.
