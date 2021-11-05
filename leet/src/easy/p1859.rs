// A sentence is a list of words that are separated by a single space with no leading or trailing spaces. Each word consists of lowercase and uppercase English letters.

// A sentence can be shuffled by appending the 1-indexed word position to each word then rearranging the words in the sentence.

//     For example, the sentence "This is a sentence" can be shuffled as "sentence4 a3 is2 This1" or "is2 sentence4 This1 a3".

// Given a shuffled sentence s containing no more than 9 words, reconstruct and return the original sentence.

// Example 1:

// Input: s = "is2 sentence4 This1 a3"
// Output: "This is a sentence"
// Explanation: Sort the words in s to their original positions "This1 is2 a3 sentence4", then remove the numbers.

// Example 2:

// Input: s = "Myself2 Me1 I4 and3"
// Output: "Me Myself and I"
// Explanation: Sort the words in s to their original positions "Me1 Myself2 and3 I4", then remove the numbers.

// Constraints:

//     2 <= s.length <= 200
//     s consists of lowercase and uppercase English letters, spaces, and digits from 1 to 9.
//     The number of words in s is between 1 and 9.
//     The words in s are separated by a single space.
//     s contains no leading or trailing spaces.
struct Solution;

impl Solution {
    pub fn sort_sentence(s: String) -> String {
        if s.len() < 2 || s.len() > 200 {
            return s;
        }

        let words: Vec<String> = s.split(" ").map(|x| x.to_string()).collect();
        let n_words = words.len();

        if n_words < 1 || n_words > 9 {
            return s;
        }

        let mut new_words: Vec<String> = vec!["".to_string(); n_words];

        for word in words.iter() {
            let chars: Vec<char> = word.chars().collect();
            let i = (chars.last().unwrap().to_digit(10).unwrap() - 1) as usize;
            new_words[i] = word[0..word.len() - 1].to_string();
        }

        let new_sentence = new_words.join(" ");

        return new_sentence;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let s = "is2 sentence4 This1 a3".to_string();
        let result = Solution::sort_sentence(s);

        assert_eq!(result, "This is a sentence");
    }
}
