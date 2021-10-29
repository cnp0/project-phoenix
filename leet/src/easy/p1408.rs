// Given an array of string words, return all strings in words which is substring of another word in any order.

// String words[i] is substring of words[j], if can be obtained removing some characters to left and/or right side of words[j].

// Example 1:

// Input: words = ["mass","as","hero","superhero"]
// Output: ["as","hero"]
// Explanation: "as" is substring of "mass" and "hero" is substring of "superhero".
// ["hero","as"] is also a valid answer.

// Example 2:

// Input: words = ["leetcode","et","code"]
// Output: ["et","code"]
// Explanation: "et", "code" are substring of "leetcode".

// Example 3:

// Input: words = ["blue","green","bu"]
// Output: []

// Constraints:

//     1 <= words.length <= 100
//     1 <= words[i].length <= 30
//     words[i] contains only lowercase English letters.
//     It's guaranteed that words[i] will be unique.

struct Solution;

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();

        if words.len() > 100 {
            return result;
        }

        for (i, w_i) in words.iter().enumerate() {
            // TODO: must be pre-check
            // if w_i.len() > 30 {
            //     return Vec::new();
            // }

            for (j, w_j) in words.iter().enumerate() {
                if i != j {
                    if w_j.contains(w_i) {
                        result.push(w_i.clone());
                    }
                }
            }
        }

        result.sort_unstable();
        result.dedup();

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_WORDS: [&str; 4] = ["mass", "as", "hero", "superhero"];
    const CORRECT_RESULT: [&str; 2] = ["as", "hero"];

    #[test]
    fn test_solution() {
        let words = TEST_WORDS.iter().map(|s| s.to_string()).collect();
        let result = Solution::string_matching(words);
        assert_eq!(result, CORRECT_RESULT);
    }
}
