// Given an array nums. We define a running sum of an array as runningSum[i] = sum(nums[0]…nums[i]).

// Return the running sum of nums.

// Example 1:

// Input: nums = [1,2,3,4]
// Output: [1,3,6,10]
// Explanation: Running sum is obtained as follows: [1, 1+2, 1+2+3, 1+2+3+4].

// Example 2:

// Input: nums = [1,1,1,1,1]
// Output: [1,2,3,4,5]
// Explanation: Running sum is obtained as follows: [1, 1+1, 1+1+1, 1+1+1+1, 1+1+1+1+1].

// Example 3:

// Input: nums = [3,1,2,10,1]
// Output: [3,4,6,16,17]

// Constraints:

//     1 <= nums.length <= 1000
//     -10^6 <= nums[i] <= 10^6
struct Solution;

impl Solution {
    // returns empty vec for invalid inputs
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();

        if len < 1 || len > 1000 {
            return Vec::new();
        }

        let mut sums: Vec<i32> = Vec::new();
        for i in 0..len {
            let num = nums[i];
            if i == 0 {
                sums.push(num);
            } else {
                sums.push(sums[i - 1] + num);
            }
        }

        return sums;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let nums = vec![1, 2, 3, 4];
        let correct_result = vec![1, 3, 6, 10];
        let result = Solution::running_sum(nums);
        assert_eq!(result, correct_result);
    }
}
