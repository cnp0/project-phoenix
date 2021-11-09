// Given an integer array nums, handle multiple queries of the following type:

//     1. Calculate the sum of the elements of nums between indices left and right inclusive where left <= right.

// Implement the NumArray class:

//     - NumArray(int[] nums) Initializes the object with the integer array nums.
//     - int sumRange(int left, int right) Returns the sum of the elements of nums between indices left and right inclusive (i.e. nums[left] + nums[left + 1] + ... + nums[right]).

// Example 1:

// Input
// ["NumArray", "sumRange", "sumRange", "sumRange"]
// [[[-2, 0, 3, -5, 2, -1]], [0, 2], [2, 5], [0, 5]]
// Output
// [null, 1, -1, -3]

// Explanation
// NumArray numArray = new NumArray([-2, 0, 3, -5, 2, -1]);
// numArray.sumRange(0, 2); // return (-2) + 0 + 3 = 1
// numArray.sumRange(2, 5); // return 3 + (-5) + 2 + (-1) = -1
// numArray.sumRange(0, 5); // return (-2) + 0 + 3 + (-5) + 2 + (-1) = -3

// Constraints:

//     1 <= nums.length <= 104
//     -105 <= nums[i] <= 105
//     0 <= left <= right < nums.length
//     At most 104 calls will be made to sumRange.
struct NumArray {
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: &Vec<i32>) -> Self {
        return NumArray { nums: nums.clone() };
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        return self.nums[(left as usize)..(right as usize) + 1]
            .iter()
            .sum();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let nums = vec![-2, 0, 3, -5, 2, -1];
        let obj = NumArray::new(&nums);
        let result: i32 = obj.sum_range(0, 2);

        assert_eq!(result, 1);
    }
}
