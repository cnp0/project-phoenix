// A bus has n stops numbered from 0 to n - 1 that form a circle. We know the distance between all pairs of neighboring stops where distance[i] is the distance between the stops number i and (i + 1) % n.

// The bus goes along both directions i.e. clockwise and counterclockwise.

// Return the shortest distance between the given start and destination stops.

// Example 1:

// [   0   ] ======1====== [   1   ]
//  (Start)                  (End)
//     |                       |
//     |                       |
//     4                       2
//     |                       |
//     |                       |
// [   3   ] ------3------ [   2   ]

// Input: distance = [1,2,3,4], start = 0, destination = 1
// Output: 1
// Explanation: Distance between 0 and 1 is 1 or 9, minimum is 1.

// Example 2:

// [   0   ] ======1====== [   1   ]
//  (Start)
//     |                       ||
//     |                       ||
//     4                       2
//     |                       ||
//     |                       ||
// [   3   ] ------3------ [   2   ]
//                           (End)

// Input: distance = [1,2,3,4], start = 0, destination = 2
// Output: 3
// Explanation: Distance between 0 and 2 is 3 or 7, minimum is 3.

// Example 3:

// [   0   ] ------1------ [   1   ]
//  (Start)
//    ||                       |
//    ||                       |
//     4                       2
//    ||                       |
//    ||                       |
// [   3   ] ------3------ [   2   ]
//   (End)

// Input: distance = [1,2,3,4], start = 0, destination = 3
// Output: 4
// Explanation: Distance between 0 and 3 is 6 or 4, minimum is 4.

// Constraints:

//     1 <= n <= 10^4
//     distance.length == n
//     0 <= start, destination < n
//     0 <= distance[i] <= 10^4
use std::cmp::{max, min};

struct Solution;

impl Solution {
    // returns 0 for invalid inputs
    pub fn distance_between_bus_stops(distance: &Vec<i32>, start: &i32, destination: &i32) -> i32 {
        if distance.len() < 1 || distance.len() > 10i32.pow(4) as usize {
            return 0;
        }

        let (min_i, max_i) = (
            min(*start, *destination) as usize,
            max(*start, *destination) as usize,
        );

        // TODO: would nee to use isize for negative check to make sense
        if min_i < 0 as usize || max_i > 10i32.pow(4) as usize {
            return 0;
        }

        let full_sum: i32 = distance.iter().sum();
        let sum_slice = distance[min_i..max_i].iter().sum();
        let sum_opposite = full_sum - sum_slice;

        return min(sum_slice, sum_opposite);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let distance = vec![1, 2, 3, 4];

        let result = Solution::distance_between_bus_stops(&distance, &0, &1);

        assert_eq!(result, 1);

        let result = Solution::distance_between_bus_stops(&distance, &0, &2);

        assert_eq!(result, 3);

        let result = Solution::distance_between_bus_stops(&distance, &0, &3);

        assert_eq!(result, 4);
    }
}
