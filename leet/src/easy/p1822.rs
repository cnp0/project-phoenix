// There is a function signFunc(x) that returns:

//     1 if x is positive.
//     -1 if x is negative.
//     0 if x is equal to 0.

// You are given an integer array nums. Let product be the product of all values in the array nums.

// Return signFunc(product).

// Example 1:

// Input: nums = [-1,-2,-3,-4,3,2,1]
// Output: 1
// Explanation: The product of all values in the array is 144, and signFunc(144) = 1

// Example 2:

// Input: nums = [1,5,0,2,-3]
// Output: 0
// Explanation: The product of all values in the array is 0, and signFunc(0) = 0

// Example 3:

// Input: nums = [-1,1,-1,1,-1]
// Output: -1
// Explanation: The product of all values in the array is -1, and signFunc(-1) = -1

// Constraints:

//     1 <= nums.length <= 1000
//     -100 <= nums[i] <= 100
struct Solution;

impl Solution {
    // returns 0 for invalid inputs
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        if nums.len() < 1 || nums.len() > 1000 || nums.contains(&0) {
            return 0;
        }

        let mut negatives = Vec::new();

        for n in nums.iter() {
            if *n < -100 || *n > 100 {
                return 0;
            }

            if *n < 0 {
                negatives.push(*n);
            }
        }

        if negatives.len() % 2 != 0 {
            return -1;
        }

        return 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let nums = vec![-1, -2, -3, -4, 3, 2, 1];
        let result = Solution::array_sign(nums);
        assert_eq!(result, 1);
    }
}
