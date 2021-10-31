// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

// An input string is valid if:

//     Open brackets must be closed by the same type of brackets.
//     Open brackets must be closed in the correct order.

// Example 1:

// Input: s = "()"
// Output: true

// Example 2:

// Input: s = "()[]{}"
// Output: true

// Example 3:

// Input: s = "(]"
// Output: false

// Example 4:

// Input: s = "([)]"
// Output: false

// Example 5:

// Input: s = "{[]}"
// Output: true

// Constraints:

//     1 <= s.length <= 104
//     s consists of parentheses only '()[]{}'.
use std::collections::HashMap;
use std::collections::LinkedList;

const OPEN_BRACKETS: [char; 3] = ['[', '{', '('];
const CLOSED_BRACKETS: [char; 3] = [']', '}', ')'];

struct Solution;

impl Solution {
    // returns false for some invalid inputs (does not check if only parentheses types)
    pub fn is_valid(s: String) -> bool {
        let len = s.len();
        if len < 1 || len > 10i32.pow(4) as usize {
            return false;
        }

        let mut lookup = HashMap::new();
        for i in 0..CLOSED_BRACKETS.len() {
            lookup.insert(CLOSED_BRACKETS[i], OPEN_BRACKETS[i]);
        }

        let mut stack = LinkedList::new();
        for c in s.chars() {
            if lookup.values().any(|x| *x == c) {
                stack.push_front(c);
            } else if lookup.contains_key(&c) {
                if stack.len() == 0 || *lookup.get(&c).unwrap() != stack.pop_front().unwrap() {
                    return false;
                }
            } else {
                return false;
            }
        }

        return stack.is_empty();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let valid_input = "([{}])".to_string();
        assert_eq!(Solution::is_valid(valid_input), true);

        let invalid_input = "(][)".to_string();
        assert_eq!(Solution::is_valid(invalid_input), false);
    }
}
