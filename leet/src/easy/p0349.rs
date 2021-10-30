// Given two integer arrays nums1 and nums2, return an array of their intersection. Each element in the result must be unique and you may return the result in any order.

// Example 1:

// Input: nums1 = [1,2,2,1], nums2 = [2,2]
// Output: [2]

// Example 2:

// Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
// Output: [9,4]
// Explanation: [4,9] is also accepted.

// Constraints:

//     1 <= nums1.length, nums2.length <= 1000
//     0 <= nums1[i], nums2[i] <= 1000
struct Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();

        if (nums1.len() < 1) | (nums1.len() > 1000) | (nums2.len() < 1) | (nums2.len() > 1000) {
            return result;
        }

        for i in nums1.iter() {
            for j in nums2.iter() {
                if i == j {
                    result.push(*i);
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

    const TEST_ARRAY_1: [i32; 4] = [1, 2, 2, 1];
    const TEST_ARRAY_2: [i32; 2] = [2, 2];
    const CORRECT_RESULT: [i32; 1] = [2];

    #[test]
    fn test_solution() {
        let result = Solution::intersection(TEST_ARRAY_1.to_vec(), TEST_ARRAY_2.to_vec());
        assert_eq!(result, CORRECT_RESULT.to_vec());
    }
}
