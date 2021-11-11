// Given the root of a binary tree with unique values and the values of two different nodes of the tree x and y, return true if the nodes corresponding to the values x and y in the tree are cousins, or false otherwise.

// Two nodes of a binary tree are cousins if they have the same depth with different parents.

// Note that in a binary tree, the root node is at the depth 0, and children of each depth k node are at the depth k + 1.

// Example 1:

//     1
//    / \
//   2   3
//  /
// 4

// Input: root = [1,2,3,4], x = 4, y = 3
// Output: false

// Example 2:

//     1
//    / \
//   2   3
//    \   \
//     4   5

// Input: root = [1,2,3,null,4,null,5], x = 5, y = 4
// Output: true

// Example 3:

//     1
//    / \
//   2   3
//    \
//     4

// Input: root = [1,2,3,null,4], x = 2, y = 3
// Output: false

// Constraints:

//     The number of nodes in the tree is in the range [2, 100].
//     1 <= Node.val <= 100
//     Each node has a unique value.
//     x != y
//     x and y are exist in the tree.
use std::cell::RefCell;
use std::rc::Rc;

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

struct Solution;

impl Solution {
    pub fn is_cousins(_root: Option<Rc<RefCell<TreeNode>>>, _x: i32, _y: i32) -> bool {
        return false;
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_solution() {}
}
