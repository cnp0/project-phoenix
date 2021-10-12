// given an array of u8s, use binary search to find a desired value

const SAMPLE_ARRAY: [u8; 10] = [9, 8, 2, 3, 4, 5, 6, 4, 1, 7];

// search implementing binary search
//   array: pre-sorted
//   TODO: immutible, recursion
fn binary_search(array: &[u8; 10], target: &u8) -> Option<usize> {
    let mut low: usize = 0;
    let mut high: usize = array.len();

    while low < high {
        let middle: usize = (high - low) / 2 + low;

        if *target < array[middle] {
            high = middle;
        } else if *target > array[middle] {
            low = middle + 1;
        } else if *target == array[middle] {
            return Some(middle);
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let mut test_array = SAMPLE_ARRAY.clone();
        let target_val = 5;

        test_array.sort();
        let i = binary_search(&test_array, &target_val);

        assert_eq!(target_val, test_array[i.unwrap()]);
    }
}
