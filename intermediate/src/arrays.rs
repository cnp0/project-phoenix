// arrays are same-type data structures stored in constiguous memory
// on the stack and must be declared with a usize (length)
const CHARACTER_ARRAY: [char; 5] = ['c', 'h', 'a', 'r', 's'];
const STRING_ARRAY: [&str; 4] = ["a", "list", "of", "strings"];
const INT_ARRAY: [i8; 4] = [0, 1, 2, 3];
const BOOL_ARRAY: [bool; 2] = [true, false];

// multi-dimensional arrays are arrays af arrays
const MULTI_DIM_ARRAY: [[i8; 4]; 4] = [[1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4]];

// intializing arrays with values and usize
const LARGE_ARRAY_USIZE: usize = 1000;
const LARGE_ARRAY: [u32; LARGE_ARRAY_USIZE] = [5; LARGE_ARRAY_USIZE];

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_arrays() {
        assert!(CHARACTER_ARRAY.len() > 0);
        assert!(CHARACTER_ARRAY.len() > 0);
        assert!(mem::size_of_val(&INT_ARRAY) > 0);
        assert!(BOOL_ARRAY.len() > 0);

        assert!(MULTI_DIM_ARRAY.len() > 0);
        assert!(MULTI_DIM_ARRAY[0].len() > 0);

        assert_eq!(LARGE_ARRAY_USIZE, LARGE_ARRAY.len());
    }
}
