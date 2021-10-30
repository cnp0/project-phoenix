// A square triple (a,b,c) is a triple where a, b, and c are integers and a^2 + b^2 = c^2.

// Given an integer n, return the number of square triples such that 1 <= a, b, c <= n.

// Example 1:

// Input: n = 5
// Output: 2
// Explanation: The square triples are (3,4,5) and (4,3,5).

// Example 2:

// Input: n = 10
// Output: 4
// Explanation: The square triples are (3,4,5), (4,3,5), (6,8,10), and (8,6,10).

// Constraints:

//     1 <= n <= 250
struct Solution;

impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        if (n < 1) | (n > 250) {
            return 0;
        }

        // convert n to a list of squared integers from 1 -> n
        let mut squares = Vec::new();
        for i in 1..n + 1 {
            squares.push(i * i);
        }

        // count
        let mut count = 0;
        for i in squares.iter() {
            for j in squares.iter() {
                if i != j {
                    let sum = *i + *j;

                    if squares.contains(&sum) {
                        count += 1;
                    }
                }
            }
        }

        return count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_NUMBER: i32 = 5;
    const CORRECT_RESULT: i32 = 2;

    #[test]
    fn test_solution() {
        let result = Solution::count_triples(TEST_NUMBER);
        assert_eq!(result, CORRECT_RESULT);
    }
}
