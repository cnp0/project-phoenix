// Find N Unique Integers Sum up to Zero
// Given an integer n, return any array containing n unique integers such that they add up to 0.
//
// Example:
// Input: n = 5
// Output: [-7,-1,1,3,4]
// Explanation: These arrays also are accepted [-5,-1,1,2,3] , [-3,-1,2,-2,4].
//
// Example:
// Input: n = 3
// Output: [-1,0,1]
//
// Example:
// Input: n = 1
// Output: [0]
//
// Constraints:
// 1 <= n <= 1000
struct Solution;

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut vec: Vec<i32> = (0..n).collect();
        let sum: i32 = vec.iter().sum();

        vec[0] = vec[0] - sum;

        return vec;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_NUMS: [i32; 5] = [5, 17, 31, 511, 999];

    // TODO: add nunique check
    #[test]
    fn test_solution() {
        for n in TEST_NUMS.iter() {
            let vec = Solution::sum_zero(*n);
            let sum: i32 = vec.iter().sum();
            assert_eq!(sum, 0);
        }
    }
}
