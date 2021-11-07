// Given an array arr of positive integers sorted in a strictly increasing order, and an integer k.

// Find the kth positive integer that is missing from this array.

// Example 1:

// Input: arr = [2,3,4,7,11], k = 5
// Output: 9
// Explanation: The missing positive integers are [1,5,6,8,9,10,12,13,...]. The 5th missing positive integer is 9.

// Example 2:

// Input: arr = [1,2,3,4], k = 2
// Output: 6
// Explanation: The missing positive integers are [5,6,7,...]. The 2nd missing positive integer is 6.

// Constraints:

//     1 <= arr.length <= 1000
//     1 <= arr[i] <= 1000
//     1 <= k <= 1000
//     arr[i] < arr[j] for 1 <= i < j <= arr.length
struct Solution;

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let len = arr.len();
        let mut count = 0;
        let mut i = 0;
        let mut last_n = 0;

        while count != k && i <= len - 1 {
            if arr[i as usize] != last_n + 1 {
                count += 1;
            } else {
                i += 1;
            }

            last_n += 1;
        }

        if last_n == arr[len - 1] {
            last_n += k - count;
        }

        return last_n;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let arr = vec![2, 3, 4, 7, 11];
        let k = 5;
        let result = Solution::find_kth_positive(arr, k);

        assert_eq!(result, 9);
    }
}
