// You are given an array coordinates, coordinates[i] = [x, y], where [x, y] represents the coordinate of a point. Check if these points make a straight line in the XY plane.

// Example 1:

//             /
//            x (6,7)
//           /
//          x (5,6)
//         /
//        x (4,5)
//       /
//      x (3,4)
//     /
//    x (2,3)
//   /
//  x (1,2)
// /

// Input: coordinates = [[1,2],[2,3],[3,4],[4,5],[5,6],[6,7]]
// Output: true

// Example 2:

//                     x (7,7)
//
//              x (5,6)
//
//           x (4,5)
//
//        x (3,4)
//
//    x (2,2)
//
//  x (1,1)

// Input: coordinates = [[1,1],[2,2],[3,4],[4,5],[5,6],[7,7]]
// Output: false

// Constraints:

//     2 <= coordinates.length <= 1000
//     coordinates[i].length == 2
//     -10^4 <= coordinates[i][0], coordinates[i][1] <= 10^4
//     coordinates contains no duplicate point.
pub struct Solution;

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        if coordinates.len() < 2 {
            return false;
        }

        let mut delta = (
            (coordinates[1][0] - coordinates[0][0]).abs(),
            (coordinates[1][1] - coordinates[0][1]).abs(),
        );

        for i in 2..coordinates.len() {
            let new_delta = (
                (coordinates[i][0] - coordinates[i - 1][0]).abs(),
                (coordinates[i][1] - coordinates[i - 1][1]).abs(),
            );

            if new_delta != delta {
                return false;
            }

            delta = new_delta;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let coordinates = vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 7],
        ];
        let result = Solution::check_straight_line(coordinates);

        assert_eq!(result, true);

        let coordinates = vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![7, 7],
        ];
        let result = Solution::check_straight_line(coordinates);

        assert_eq!(result, false);
    }
}
