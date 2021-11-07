// Given an array of distinct integers candidates and a target integer target, return a list of all unique combinations of candidates where the chosen numbers sum to target. You may return the combinations in any order.

// The same number may be chosen from candidates an unlimited number of times. Two combinations are unique if the frequency of at least one of the chosen numbers is different.

// It is guaranteed that the number of unique combinations that sum up to target is less than 150 combinations for the given input.

// Example 1:

// Input: candidates = [2,3,6,7], target = 7
// Output: [[2,2,3],[7]]
// Explanation:
// 2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
// 7 is a candidate, and 7 = 7.
// These are the only two combinations.

// Example 2:

// Input: candidates = [2,3,5], target = 8
// Output: [[2,2,2,2],[2,3,3],[3,5]]

// Example 3:

// Input: candidates = [2], target = 1
// Output: []

// Example 4:

// Input: candidates = [1], target = 1
// Output: [[1]]

// Example 5:

// Input: candidates = [1], target = 2
// Output: [[1,1]]

// Constraints:

//     1 <= candidates.length <= 30
//     1 <= candidates[i] <= 200
//     All elements of candidates are distinct.
//     1 <= target <= 500

// womp. getting stomped by rust.
// following https://leetcode.com/problems/combination-sum/discuss/1416367/Rust-Simple-efficient-backtracking
// say no to brute force, kids.
// TODO: play with dfs more

struct Solution;

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        return find(&candidates, 0, target);
    }
}

fn find(sorted_candidates: &[i32], i: usize, target: i32) -> Vec<Vec<i32>> {
    if target == 0 {
        return vec![vec![]];
    }

    let end_i = match sorted_candidates.binary_search(&target) {
        Ok(x) => x + 1,
        Err(x) => x,
    };

    let mut results = Vec::new();
    for (i, &n) in sorted_candidates[..end_i].iter().enumerate().skip(i) {
        let need = target - n;
        for mut set in find(sorted_candidates, i, need).into_iter() {
            set.push(n);
            results.push(set);
        }
    }
    return results;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let target = 8;
        let candidates = vec![2, 3, 5];
        let result = Solution::combination_sum(candidates, target);

        assert_eq!(result, vec![vec![2, 2, 2, 2], vec![3, 3, 2], vec![5, 3]]);
    }
}
