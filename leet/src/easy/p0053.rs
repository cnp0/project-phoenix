// Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.

// A subarray is a contiguous part of an array.

// Example 1:

// Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
// Output: 6
// Explanation: [4,-1,2,1] has the largest sum = 6.

// Example 2:

// Input: nums = [1]
// Output: 1

// Example 3:

// Input: nums = [5,4,-1,7,8]
// Output: 23

// Constraints:

//     1 <= nums.length <= 105
//     -104 <= nums[i] <= 104
struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_cummulative = i32::MIN;
        let mut max_at = 0;

        for i in 0..nums.len() {
            max_at = max_at + nums[i];

            if max_cummulative < max_at {
                max_cummulative = max_at;
            }

            if max_at < 0 {
                max_at = 0;
            }
        }

        return max_cummulative;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let correct_result = 6;
        let result = Solution::max_sub_array(nums);
        assert_eq!(result, correct_result);
    }
}
