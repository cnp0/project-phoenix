// Given an integer array nums and an integer k, return the number of pairs (i, j) where i < j such that |nums[i] - nums[j]| == k.

// The value of |x| is defined as:

//     x if x >= 0.
//     -x if x < 0.

// Example 1:

// Input: nums = [1,2,2,1], k = 1
// Output: 4
// Explanation: The pairs with an absolute difference of 1 are:
// - [1,2,2,1]
// - [1,2,2,1]
// - [1,2,2,1]
// - [1,2,2,1]

// Example 2:

// Input: nums = [1,3], k = 3
// Output: 0
// Explanation: There are no pairs with an absolute difference of 3.

// Example 3:

// Input: nums = [3,2,1,5,4], k = 2
// Output: 3
// Explanation: The pairs with an absolute difference of 2 are:
// - [3,2,1,5,4]
// - [3,2,1,5,4]
// - [3,2,1,5,4]

// Constraints:

//     1 <= nums.length <= 200
//     1 <= nums[i] <= 100
//     1 <= k <= 99
struct Solution;

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            for j in i..nums.len() {
                if (nums[i] - nums[j]).abs() == k {
                    count += 1;
                }
            }
        }

        return count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let nums = vec![1, 2, 2, 1];
        let k = 1;
        let result = Solution::count_k_difference(nums, k);

        assert_eq!(result, 4);

        let nums = vec![1, 3];
        let k = 3;
        let results = Solution::count_k_difference(nums, k);

        assert_eq!(results, 0);

        let nums = vec![3, 2, 1, 5, 4];
        let k = 2;
        let results = Solution::count_k_difference(nums, k);

        assert_eq!(results, 3);
    }
}
