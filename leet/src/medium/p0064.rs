// Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right, which minimizes the sum of all numbers along its path.

// Note: You can only move either down or right at any point in time.

// Example 1:

// -------------------
// |  1  |  3  |  1  |
// |  1  |  5  |  1  |
// |  4  |  2  |  1  |
// -------------------

// Input: grid = [[1,3,1],[1,5,1],[4,2,1]]
// Output: 7
// Explanation: Because the path 1 → 3 → 1 → 1 → 1 minimizes the sum.

// Example 2:

// Input: grid = [[1,2,3],[4,5,6]]
// Output: 12

// Constraints:

//     m == grid.length
//     n == grid[i].length
//     1 <= m, n <= 200
//     0 <= grid[i][j] <= 100
use std::cmp::min;

struct Solution;

impl Solution {
    // returns 0 for invalid inputs
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let col_len = grid.len();
        if col_len < 1 || col_len > 200 {
            return 0;
        }

        let row_len = grid[0].len();
        if row_len < 1 || row_len > 200 {
            return 0;
        }

        let mut new_grid = grid.clone();

        for i in 1..row_len {
            new_grid[0][i] = new_grid[0][i - 1] + new_grid[0][i];
        }

        for i in 1..col_len {
            new_grid[i][0] = new_grid[i - 1][0] + new_grid[i][0];
        }

        for i in 1..col_len {
            for j in 1..row_len {
                new_grid[i][j] = min(new_grid[i - 1][j], new_grid[i][j - 1]) + new_grid[i][j];
            }
        }

        let result = new_grid[col_len - 1][row_len - 1];

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        let correct_result = 7;
        let result = Solution::min_path_sum(grid);
        assert_eq!(result, correct_result);
    }
}
