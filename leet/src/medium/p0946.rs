// Given two integer arrays pushed and popped each with distinct values, return true if this could have been the result of a sequence of push and pop operations on an initially empty stack, or false otherwise.

// Example 1:

// Input: pushed = [1,2,3,4,5], popped = [4,5,3,2,1]
// Output: true
// Explanation: We might do the following sequence:
// push(1), push(2), push(3), push(4),
// pop() -> 4,
// push(5),
// pop() -> 5, pop() -> 3, pop() -> 2, pop() -> 1

// Example 2:

// Input: pushed = [1,2,3,4,5], popped = [4,3,5,1,2]
// Output: false
// Explanation: 1 cannot be popped before 2.

// Constraints:

//     1 <= pushed.length <= 1000
//     0 <= pushed[i] <= 1000
//     All the elements of pushed are unique.
//     popped.length == pushed.length
//     popped is a permutation of pushed.
use std::collections::VecDeque;

struct Solution;

impl Solution {
    // returns false for invalid inputs
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let pushed_len = pushed.len();
        if pushed_len < 1 || pushed_len > 1000 || pushed_len != popped.len() {
            return false;
        }

        // i, j for pushed, popped indices
        let (mut i, mut j) = (0, 0);
        let mut stack: VecDeque<i32> = VecDeque::new();
        while i < pushed_len || j < pushed_len {
            if !stack.is_empty() && stack[0] == popped[j] {
                let _p = stack.pop_front();
                j += 1;
            } else if i < pushed_len {
                stack.push_front(pushed[i]);
                i += 1;
            } else {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let pushed = vec![1, 2, 3, 4, 5];
        let popped = vec![4, 5, 3, 2, 1];
        let result = Solution::validate_stack_sequences(pushed, popped);

        assert_eq!(result, true);
    }
}
