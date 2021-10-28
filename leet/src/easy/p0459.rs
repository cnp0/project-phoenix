// Given a string s, check if it can be constructed by taking a substring of it and appending multiple copies of the substring together.

// Example 1:

// Input: s = "abab"
// Output: true
// Explanation: It is the substring "ab" twice.

// Example 2:

// Input: s = "aba"
// Output: false

// Example 3:

// Input: s = "abcabcabcabc"
// Output: true
// Explanation: It is the substring "abc" four times or the substring "abcabc" twice.

// Constraints:

// 1 <= s.length <= 10^4
// s consists of lowercase English letters.
struct Solution;

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let len = s.len();

        if (len <= 1) | (len > 10i32.pow(4) as usize) {
            return false;
        }

        // len mod distinct character count should be 0 to be a candidate
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();
        chars.dedup();

        let count_distinct = chars.len();

        if len % count_distinct != 0 {
            return false;
        }

        for c_i in chars.iter() {
            let mut last_position = 0;
            let mut start_counting = false;

            for (i, c_j) in s.chars().enumerate() {
                if *c_i == c_j {
                    if (start_counting) & (i - last_position != count_distinct) {
                        return false;
                    }

                    last_position = i;
                    start_counting = true;
                }
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_GOOD_STRING: &str = "abcabcabc";
    const TEST_BAD_STRING: &str = "aslk;djsa";

    // TODO: apparently accepted input
    // const TEST_GOOD_STRING: &str = "abaababaab";

    #[test]
    fn test_solution() {
        let result = Solution::repeated_substring_pattern(TEST_GOOD_STRING.to_string());
        assert!(result);

        let result = Solution::repeated_substring_pattern(TEST_BAD_STRING.to_string());
        assert!(!result);
    }
}
