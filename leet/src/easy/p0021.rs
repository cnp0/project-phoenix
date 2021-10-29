// Merge two sorted linked lists and return it as a sorted list. The list should be made by splicing together the nodes of the first two lists.

// Example 1:

// Input: l1 = [1,2,4], l2 = [1,3,4]
// Output: [1,1,2,3,4,4]

// Example 2:

// Input: l1 = [], l2 = []
// Output: []

// Example 3:

// Input: l1 = [], l2 = [0]
// Output: [0]

// Constraints:

//     The number of nodes in both lists is in the range [0, 50].
//     -100 <= Node.val <= 100
//     Both l1 and l2 are sorted in non-decreasing order.

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn merge_two_lists(
        _l1: Option<Box<ListNode>>,
        _l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_solution() {
        // TODO: look into Option.get_or_insert()

        // let test_list_1: Option<Box<ListNode>> = Some(Box::new(ListNode {
        //     val: 1,
        //     next: Some(Box::new(ListNode {
        //         val: 2,
        //         next: Some(Box::new(ListNode { val: 4, next: None })),
        //     })),
        // }));

        // let test_list_2: Option<Box<ListNode>> = Some(Box::new(ListNode {
        //     val: 1,
        //     next: Some(Box::new(ListNode {
        //         val: 3,
        //         next: Some(Box::new(ListNode { val: 4, next: None })),
        //     })),
        // }));

        // let correct_result: Option<Box<ListNode>> = Some(Box::new(ListNode {
        //     val: 1,
        //     next: Some(Box::new(ListNode {
        //         val: 2,
        //         next: Some(Box::new(ListNode {
        //             val: 3,
        //             next: Some(Box::new(ListNode { val: 4, next: None })),
        //         })),
        //     })),
        // }));

        // let result = Solution::merge_two_lists(test_list_1, test_list_2);
        // assert_eq!(result, correct_result);
    }
}
