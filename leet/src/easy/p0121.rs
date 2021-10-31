// You are given an array prices where prices[i] is the price of a given stock on the ith day.

// You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.

// Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.

// Example 1:

// Input: prices = [7,1,5,3,6,4]
// Output: 5
// Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
// Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.

// Example 2:

// Input: prices = [7,6,4,3,1]
// Output: 0
// Explanation: In this case, no transactions are done and the max profit = 0.

// Constraints:

//     1 <= prices.length <= 105
//     0 <= prices[i] <= 104
use std::cmp::{max, min};

struct Solution;

impl Solution {
    // returns 0 for invalid inputs
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // if prices.len() < 1 || prices.len() > 105 {
        //     return 0;
        // }

        let (mut max_profit, mut min_price) = (0, i32::MAX);

        for price in prices.iter() {
            // if *buy < 0 || *buy > 104 {
            //     return 0;
            // }
            min_price = min(min_price, *price);
            let profit = *price - min_price;
            max_profit = max(max_profit, profit);
        }

        return max_profit;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let correct_result = 5;
        let result = Solution::max_profit(prices);

        assert_eq!(result, correct_result);
    }
}
