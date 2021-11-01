// You are given an integer array score of size n, where score[i] is the score of the ith athlete in a competition. All the scores are guaranteed to be unique.

// The athletes are placed based on their scores, where the 1st place athlete has the highest score, the 2nd place athlete has the 2nd highest score, and so on. The placement of each athlete determines their rank:

//     The 1st place athlete's rank is "Gold Medal".
//     The 2nd place athlete's rank is "Silver Medal".
//     The 3rd place athlete's rank is "Bronze Medal".
//     For the 4th place to the nth place athlete, their rank is their placement number (i.e., the xth place athlete's rank is "x").

// Return an array answer of size n where answer[i] is the rank of the ith athlete.

// Example 1:

// Input: score = [5,4,3,2,1]
// Output: ["Gold Medal","Silver Medal","Bronze Medal","4","5"]
// Explanation: The placements are [1st, 2nd, 3rd, 4th, 5th].

// Example 2:

// Input: score = [10,3,8,9,4]
// Output: ["Gold Medal","5","Bronze Medal","Silver Medal","4"]
// Explanation: The placements are [1st, 5th, 3rd, 2nd, 4th].

// Constraints:

//     n == score.length
//     1 <= n <= 104
//     0 <= score[i] <= 106
//     All the values in score are unique.
use std::collections::HashMap;

const TOP_AWARDS: [&str; 3] = ["Gold Medal", "Silver Medal", "Bronze Medal"];

struct Solution;

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut sorted_scores = score.clone();
        sorted_scores.sort_by(|a, b| b.cmp(a));

        let mut awards: Vec<String> = TOP_AWARDS.iter().map(|v| v.to_string()).collect();
        let bottom_awards: Vec<String> = (4..(score.len() + 1))
            .into_iter()
            .map(|v| v.to_string())
            .collect();
        awards.extend(bottom_awards);

        let mut mapped_scores: HashMap<i32, usize> = HashMap::new();
        for (i, val) in score.iter().enumerate() {
            mapped_scores.insert(*val, i);
        }

        let mut results = vec!["".to_string(); score.len()];
        for (i, val) in sorted_scores.iter().enumerate() {
            let original_index = *mapped_scores.get(&val).unwrap();
            results[original_index] = awards[i].clone();
        }

        return results;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let score = vec![5, 4, 3, 2, 1];
        let correct_result = vec![
            "Gold Medal".to_string(),
            "Silver Medal".to_string(),
            "Bronze Medal".to_string(),
            "4".to_string(),
            "5".to_string(),
        ];
        let result = Solution::find_relative_ranks(score);

        assert_eq!(result, correct_result);
    }
}
