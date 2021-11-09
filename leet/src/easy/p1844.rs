// You are given a 0-indexed string s that has lowercase English letters in its even indices and digits in its odd indices.

// There is a function shift(c, x), where c is a character and x is a digit, that returns the xth character after c.

//     For example, shift('a', 5) = 'f' and shift('x', 0) = 'x'.

// For every odd index i, you want to replace the digit s[i] with shift(s[i-1], s[i]).

// Return s after replacing all digits. It is guaranteed that shift(s[i-1], s[i]) will never exceed 'z'.

// Example 1:

// Input: s = "a1c1e1"
// Output: "abcdef"
// Explanation: The digits are replaced as follows:
// - s[1] -> shift('a',1) = 'b'
// - s[3] -> shift('c',1) = 'd'
// - s[5] -> shift('e',1) = 'f'

// Example 2:

// Input: s = "a1b2c3d4e"
// Output: "abbdcfdhe"
// Explanation: The digits are replaced as follows:
// - s[1] -> shift('a',1) = 'b'
// - s[3] -> shift('b',2) = 'd'
// - s[5] -> shift('c',3) = 'f'
// - s[7] -> shift('d',4) = 'h'

// Constraints:

//     1 <= s.length <= 100
//     s consists only of lowercase English letters and digits.
//     shift(s[i-1], s[i]) <= 'z' for all odd indices i.
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn replace_digits(s: String) -> String {
        let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        let mut lookup = HashMap::new();

        for (i, c) in alphabet.iter().enumerate() {
            lookup.insert(*c, i);
        }

        let s_bytes = s.as_bytes();
        let mut new_s = "".to_string();

        for (i, c) in s_bytes.iter().enumerate() {
            if i % 2 != 0 {
                // index position of previous character
                let j = *lookup.get(&(s_bytes[i - 1] as char)).unwrap();
                // current character as integer
                let k = (*c as char).to_digit(10).unwrap();
                new_s.push(alphabet[(j + k as usize) % 26]);
            } else {
                new_s.push(*c as char);
            }
        }

        return new_s;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let s = "a1c1e1".to_string();
        let result = Solution::replace_digits(s);

        assert_eq!(result, "abcdef".to_string());
    }
}
