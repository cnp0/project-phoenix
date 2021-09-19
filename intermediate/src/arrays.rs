// arrays are same-type data structures stored in constiguous memory
// on the stack and must be declared with a usize (length)
const CHARACTER_ARRAY: [char; 5] = ['c', 'h', 'a', 'r', 's'];
const INT_ARRAY: [i8; 4] = [0, 1, 2, 3];
const BOOL_ARRAY: [bool; 2] = [true, false];

// multi-dimensional arrays are arrays af arrays
const MULTI_DIM_ARRAY: [[i8; 4]; 4] = [[1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4]];

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_arrays() {
        assert!(CHARACTER_ARRAY.len() > 0);
        assert!(mem::size_of_val(&INT_ARRAY) > 0);
        assert!(BOOL_ARRAY.len() > 0);

        assert!(MULTI_DIM_ARRAY.len() > 0);
        assert!(MULTI_DIM_ARRAY[0].len() > 0);
    }
}
