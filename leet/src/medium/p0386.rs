// Given an integer n, return all the numbers in the range [1, n] sorted in lexicographical order.

// You must write an algorithm that runs in O(n) time and uses O(1) extra space.

// Example 1:

// Input: n = 13
// Output: [1,10,11,12,13,2,3,4,5,6,7,8,9]

// Example 2:

// Input: n = 2
// Output: [1,2]

// Constraints:

//     1 <= n <= 5 * 104
struct Solution;

impl Solution {
    // returns numerically ascending sorted vec for invalid inputs
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let range: Vec<i32> = (1..n + 1).collect();
        if n < 1 || n > 5 * 104 {
            return range;
        }

        let mut strings: Vec<String> = range.iter().map(|x| x.to_string()).collect();
        strings.sort();

        let result: Vec<i32> = strings.iter().map(|x| x.parse().unwrap()).collect();

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tess_solution() {
        let n = 13;
        let result = Solution::lexical_order(n);

        assert_eq!(result, vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
