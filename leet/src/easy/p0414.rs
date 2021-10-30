// Given an integer array nums, return the third distinct maximum number in this array. If the third maximum does not exist, return the maximum number.

// Example 1:

// Input: nums = [3,2,1]
// Output: 1
// Explanation:
// The first distinct maximum is 3.
// The second distinct maximum is 2.
// The third distinct maximum is 1.

// Example 2:

// Input: nums = [1,2]
// Output: 2
// Explanation:
// The first distinct maximum is 2.
// The second distinct maximum is 1.
// The third distinct maximum does not exist, so the maximum (2) is returned instead.

// Example 3:

// Input: nums = [2,2,3,1]
// Output: 1
// Explanation:
// The first distinct maximum is 3.
// The second distinct maximum is 2 (both 2's are counted together since they have the same value).
// The third distinct maximum is 1.

// Constraints:

//     1 <= nums.length <= 104
//     -231 <= nums[i] <= 231 - 1
struct Solution;

impl Solution {
    // returns 0 for invalid input
    pub fn third_max(nums: Vec<i32>) -> i32 {
        // apparently nums.len() of 1000+ is a valid input
        // if nums.len() < 1 || nums.len() > 104 {
        //     return 0;
        // }

        let mut distinct_nums = nums.clone();
        distinct_nums.sort_unstable();
        distinct_nums.dedup();

        // check if valid input
        // nvm... apparently [-2147483648,1,1] is a valid input
        // for n in distinct_nums.iter() {
        //     if *n < -231 || *n > 230 {
        //         return 0;
        //     }
        // }

        let count_distinct = distinct_nums.len();
        if count_distinct >= 3 {
            return distinct_nums[count_distinct - 3];
        }

        return distinct_nums.last().unwrap().clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let nums = vec![3, 2, 1];
        let result = Solution::third_max(nums);
        assert_eq!(result, 1);
    }
}
