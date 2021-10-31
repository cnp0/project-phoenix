// Given two integer arrays startTime and endTime and given an integer queryTime.

// The ith student started doing their homework at the time startTime[i] and finished it at time endTime[i].

// Return the number of students doing their homework at time queryTime. More formally, return the number of students where queryTime lays in the interval [startTime[i], endTime[i]] inclusive.

// Example 1:

// Input: startTime = [1,2,3], endTime = [3,2,7], queryTime = 4
// Output: 1
// Explanation: We have 3 students where:
// The first student started doing homework at time 1 and finished at time 3 and wasn't doing anything at time 4.
// The second student started doing homework at time 2 and finished at time 2 and also wasn't doing anything at time 4.
// The third student started doing homework at time 3 and finished at time 7 and was the only student doing homework at time 4.

// Example 2:

// Input: startTime = [4], endTime = [4], queryTime = 4
// Output: 1
// Explanation: The only student was doing their homework at the queryTime.

// Example 3:

// Input: startTime = [4], endTime = [4], queryTime = 5
// Output: 0

// Example 4:

// Input: startTime = [1,1,1,1], endTime = [1,3,2,4], queryTime = 7
// Output: 0

// Example 5:

// Input: startTime = [9,8,7,6,5,4,3,2,1], endTime = [10,10,10,10,10,10,10,10,10], queryTime = 5
// Output: 5

// Constraints:

//     startTime.length == endTime.length
//     1 <= startTime.length <= 100
//     1 <= startTime[i] <= endTime[i] <= 1000
//     1 <= queryTime <= 1000
struct Solution;

impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        if start_time.len() != end_time.len()
            || start_time.len() > 100
            || query_time < 1
            || query_time > 1000
        {
            return 0;
        }

        let mut count = 0;
        for i in 0..start_time.len() {
            if query_time >= start_time[i] && query_time <= end_time[i] {
                count += 1;
            }
        }

        return count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let start_time = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let end_time = vec![10, 10, 10, 10, 10, 10, 10, 10, 10];
        let query_time = 5;

        let result = Solution::busy_student(start_time, end_time, query_time);
        assert_eq!(result, 5);
    }
}
