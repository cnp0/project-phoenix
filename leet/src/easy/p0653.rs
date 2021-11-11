// Given the root of a Binary Search Tree and a target number k, return true if there exist two elements in the BST such that their sum is equal to the given target.

// Example 1:

//        5
//       / \
//      3   6
//     / \   \
//    2   4   7

// Input: root = [5,3,6,2,4,null,7], k = 9
// Output: true

// Example 2:

//        5
//       / \
//      3   6
//     / \   \
//    2   4   7

// Input: root = [5,3,6,2,4,null,7], k = 28
// Output: false

// Example 3:

// Input: root = [2,1,3], k = 4
// Output: true

// Example 4:

// Input: root = [2,1,3], k = 1
// Output: false

// Example 5:

// Input: root = [2,1,3], k = 3
// Output: true

// Constraints:

//     The number of nodes in the tree is in the range [1, 104].
//     -104 <= Node.val <= 104
//     root is guaranteed to be a valid binary search tree.
//     -105 <= k <= 105

// TODO: properly use std::rc::Rc and follow idomatic ownership
use std::cell::RefCell;
use std::collections::HashSet;
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
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut vals = HashSet::new();

        return search(&root, &mut vals, &k);
    }
}

fn search(node: &Option<Rc<RefCell<TreeNode>>>, vals: &mut HashSet<i32>, k: &i32) -> bool {
    if node.is_none() {
        return false;
    }

    // check if delta between k and node.val exists in the set of all found
    let d = *k - node.as_ref().unwrap().borrow().val;
    if vals.contains(&d) {
        return true;
    }

    vals.insert(d);

    return search(&node.as_ref().unwrap().borrow().left, vals, &k)
        || search(&node.as_ref().unwrap().borrow().right, vals, &k);
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_solution() {}
}
