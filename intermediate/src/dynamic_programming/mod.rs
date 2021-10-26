pub mod largest_contiguous_subarray;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_contiguous_subarray() {
        assert_eq!(largest_contiguous_subarray::ARRAY_SIZE, 10);

        let test_array: [i32; 10] = [-1, -3, 4, 5, -2, 1, 3, -5, 10, 9];
        let expected_result = 25;

        let result = largest_contiguous_subarray::calculate_largest_subarray(test_array);

        assert_eq!(result, expected_result);
    }
}
