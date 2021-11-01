// A parentheses string is valid if and only if:

//     It is the empty string,
//     It can be written as AB (A concatenated with B), where A and B are valid strings, or
//     It can be written as (A), where A is a valid string.

// You are given a parentheses string s. In one move, you can insert a parenthesis at any position of the string.

//     For example, if s = "()))", you can insert an opening parenthesis to be "(()))" or a closing parenthesis to be "())))".

// Return the minimum number of moves required to make s valid.

// Example 1:

// Input: s = "())"
// Output: 1

// Example 2:

// Input: s = "((("
// Output: 3

// Example 3:

// Input: s = "()"
// Output: 0

// Example 4:

// Input: s = "()))(("
// Output: 4

// Constraints:

//     1 <= s.length <= 1000
//     s[i] is either '(' or ')'.
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut stack: VecDeque<char> = VecDeque::new();
        let mut incomplete_parenetheses: Vec<char> = Vec::new();

        for c in s.chars() {
            if c == '(' {
                stack.push_front(c);
            } else if c == ')' {
                if stack.is_empty() || stack.pop_front().unwrap() != '(' {
                    incomplete_parenetheses.push(c);
                }
            } else {
                incomplete_parenetheses.push(c);
            }
        }

        return (stack.len() + incomplete_parenetheses.len()) as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let s = "()))((".to_string();
        let result = Solution::min_add_to_make_valid(s);

        assert_eq!(result, 4);
    }
}
