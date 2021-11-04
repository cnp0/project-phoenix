// Given the radius and the position of the center of a circle, implement the function randPoint which generates a uniform random point inside the circle.

// Implement the Solution class:

//     Solution(double radius, double x_center, double y_center) initializes the object with the radius of the circle radius and the position of the center (x_center, y_center).
//     randPoint() returns a random point inside the circle. A point on the circumference of the circle is considered to be in the circle. The answer is returned as an array [x, y].

// Example 1:

// Input
// ["Solution", "randPoint", "randPoint", "randPoint"]
// [[1.0, 0.0, 0.0], [], [], []]
// Output
// [null, [-0.02493, -0.38077], [0.82314, 0.38945], [0.36572, 0.17248]]

// Explanation
// Solution solution = new Solution(1.0, 0.0, 0.0);
// solution.randPoint(); // return [-0.02493, -0.38077]
// solution.randPoint(); // return [0.82314, 0.38945]
// solution.randPoint(); // return [0.36572, 0.17248]
use rand::{thread_rng, Rng};
use std::f64::consts::PI;

struct Solution {
    center: (f64, f64),
    radius: f64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        return Solution {
            center: (x_center, y_center),
            radius,
        };
    }

    fn rand_point(&self) -> Vec<f64> {
        let mut rng = thread_rng();
        
        let len = rng.gen::<f64>().sqrt() * self.radius;
        let degrees = rng.gen::<f64>() * 2.0 * PI;
        let x = self.center.0 + len * degrees.cos();
        let y = self.center.1 + len * degrees.sin();

        return vec![x, y];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let obj = Solution::new(1.0, 0.0, 0.0);
        let result = obj.rand_point();

        assert!(result[0] <= 1.0 && result[1] <= 1.0);
        assert!(result[0] >= -1.0 && result[1] >= -1.0);
    }
}
