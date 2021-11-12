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

// TODO: properly use std::rc::Rc and follow idomatic ownership
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

struct Solution {
    vals: Vec<(Rc<RefCell<TreeNode>>, i32)>,
}

impl Solution {
    pub fn new() -> Solution {
        Solution { vals: Vec::new() }
    }

    pub fn is_cousins(&mut self, root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        self.search(&root, &None, &0, &x, &y);

        let (node_x, node_y) = (&self.vals[0], &self.vals[1]);

        return *node_x.0 != *node_y.0 && node_x.1 == node_y.1;
    }

    fn search(
        &mut self,
        node: &Option<Rc<RefCell<TreeNode>>>,
        parent: &Option<Rc<RefCell<TreeNode>>>,
        depth: &i32,
        x: &i32,
        y: &i32,
    ) {
        if node.is_none() {
            return;
        }

        // check if delta between k and node.val exists in the set of all found
        if node.as_ref().unwrap().borrow().val == *x || node.as_ref().unwrap().borrow().val == *y {
            self.vals.push((node.as_ref().unwrap().clone(), *depth));
        }

        let new_depth = depth + 1;

        self.search(
            &node.as_ref().unwrap().borrow().left,
            parent,
            &new_depth,
            x,
            y,
        );
        self.search(
            &node.as_ref().unwrap().borrow().right,
            parent,
            &new_depth,
            x,
            y,
        );
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_solution() {}
}
