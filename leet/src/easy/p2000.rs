// Given a 0-indexed string word and a character ch, reverse the segment of word that starts at index 0 and ends at the index of the first occurrence of ch (inclusive). If the character ch does not exist in word, do nothing.

//     For example, if word = "abcdefd" and ch = "d", then you should reverse the segment that starts at 0 and ends at 3 (inclusive). The resulting string will be "dcbaefd".

// Return the resulting string.

// Example 1:

// Input: word = "abcdefd", ch = "d"
// Output: "dcbaefd"
// Explanation: The first occurrence of "d" is at index 3.
// Reverse the part of word from 0 to 3 (inclusive), the resulting string is "dcbaefd".

// Example 2:

// Input: word = "xyxzxe", ch = "z"
// Output: "zxyxxe"
// Explanation: The first and only occurrence of "z" is at index 3.
// Reverse the part of word from 0 to 3 (inclusive), the resulting string is "zxyxxe".

// Example 3:

// Input: word = "abcd", ch = "z"
// Output: "abcd"
// Explanation: "z" does not exist in word.
// You should not do any reverse operation, the resulting string is "abcd".

// Constraints:

//     1 <= word.length <= 250
//     word consists of lowercase English letters.
//     ch is a lowercase English letter.
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let len = word.len();

        if len < 1 || len > 250 || !word.contains(ch) {
            return word;
        }

        let word_bytes = word.as_bytes();
        let mut reversed_prefix: VecDeque<char> = VecDeque::new();
        let mut ch_index = 0;
        for i in 0..len {
            let ch_i = word_bytes[i].clone() as char;

            if ch_i == ch {
                reversed_prefix.push_front(ch_i);
                ch_index = i;

                break;
            }

            reversed_prefix.push_front(ch_i);
        }

        let mut result: String = Vec::from(reversed_prefix).into_iter().collect();
        result.push_str(&word[ch_index + 1..len]);

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let word = "abcdefgh".to_string();
        let ch = 'd';
        let correct_result = "dcbaefgh".to_string();

        let result = Solution::reverse_prefix(word, ch);
        assert_eq!(result, correct_result);
    }
}
