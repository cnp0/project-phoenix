// largest contiguous subarray
pub const ARRAY_SIZE: usize = 10;

pub fn calculate_largest_subarray(array: [i32; ARRAY_SIZE]) -> usize {
    let mut max_cummulative = i32::MIN;
    let mut max_at = 0;

    for i in 0..array.len() {
        max_at = max_at + array[i];

        if max_cummulative < max_at {
            max_cummulative = max_at;
        }

        if max_at < 0 {
            max_at = 0;
        }
    }

    return max_cummulative as usize;
}
