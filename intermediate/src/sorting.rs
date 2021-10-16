// most from https://github.com/TheAlgorithms/Rust/blob/master/src/sorting
// TODO: implement immutable versions
use std::cmp;

fn merge<T: Ord + Copy>(vec: &mut Vec<T>, low: usize, middle: usize, high: usize) -> &Vec<T> {
    let mut left_half = Vec::new();
    let mut right_half = Vec::new();

    for v in vec.iter().take(middle + 1).skip(low) {
        left_half.push(*v);
    }
    for v in vec.iter().take(high + 1).skip(middle + 1) {
        right_half.push(*v);
    }

    let left_len = left_half.len();
    let right_len = right_half.len();

    let mut l_i = 0;
    let mut r_i = 0;
    let mut a_i = low;

    // get smaller of left or right halves
    while l_i < left_len && r_i < right_len {
        if left_half[l_i] < right_half[r_i] {
            vec[a_i] = left_half[l_i];
            l_i += 1;
        } else {
            vec[a_i] = right_half[r_i];
            r_i += 1;
        }

        a_i += 1;
    }

    // handle the remaining values
    while l_i < left_len {
        vec[a_i] = left_half[l_i];
        l_i += 1;
        a_i += 1;
    }

    while r_i < right_len {
        vec[a_i] = right_half[r_i];
        r_i += 1;
        a_i += 1;
    }

    return vec;
}

fn inner_merge_sort<T: Ord + Copy>(vec: &mut Vec<T>, low: usize, high: usize) -> &Vec<T> {
    if low >= high {
        return vec;
    }

    let middle = low + (high - low) / 2;

    inner_merge_sort(vec, low, middle);
    inner_merge_sort(vec, middle + 1, high);

    let new_vec = merge(vec, low, middle, high);

    return new_vec;
}

pub fn merge_sort<T: Ord + Copy>(vec: &mut Vec<T>) -> &Vec<T> {
    let len = vec.len();
    if len <= 1 {
        return vec;
    }

    let new_vec = inner_merge_sort(vec, 0, len - 1);

    return new_vec;
}

fn partition<T: Ord>(vec: &mut Vec<T>, low: isize, high: isize) -> isize {
    let pivot = high;
    let mut i = low - 1;
    let mut j = high;

    loop {
        i += 1;
        while vec[i as usize] < vec[pivot as usize] {
            i += 1;
        }

        j -= 1;
        while j >= 0 && vec[j as usize] > vec[pivot as usize] {
            j -= 1;
        }

        if i >= j {
            break;
        } else {
            vec.swap(i as usize, j as usize);
        }
    }

    vec.swap(i as usize, pivot as usize);

    return i;
}

fn inner_quick_sort<T: Ord>(vec: &mut Vec<T>, low: isize, high: isize) {
    if low >= high {
        return;
    }

    let p_i = partition(vec, low, high);

    inner_quick_sort(vec, low, p_i - 1);
    inner_quick_sort(vec, p_i + 1, high);
}

pub fn quick_sort<T: Ord>(vec: &mut Vec<T>) {
    let len = vec.len();

    inner_quick_sort(vec, 0, (len - 1) as isize);
}

pub fn bubble_sort<T: Ord>(vec: &mut Vec<T>) -> &Vec<T> {
    for i in 0..vec.len() {
        for j in 0..vec.len() - 1 - i {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
            }
        }
    }

    return vec;
}

pub fn insertion_sort<T: cmp::PartialEq + cmp::PartialOrd + Clone>(vec: &Vec<T>) -> Vec<T> {
    let mut result = Vec::with_capacity(vec.len());

    for ele in vec.iter().cloned() {
        let n_inserted = result.len();

        // iterate over "up-to" n_inserted
        for i in 0..=n_inserted {
            if i == n_inserted || result[i] > ele {
                // insert and move others higher
                result.insert(i, ele);
                break;
            }
        }
    }

    return result;
}

pub fn selection_sort<T: Ord>(vec: &mut Vec<T>) {
    let len = vec.len();

    for i in 0..len {
        let mut min_i = i;

        for j in (i + 1)..len {
            if vec[j] < vec[min_i] {
                min_i = j;
            }
        }

        vec.swap(min_i, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ARRAY: [u8; 10] = [10, 1, 2, 3, 4, 6, 5, 9, 8, 7];

    #[test]
    fn test_sorting() {
        let mut unsorted_vec = Vec::from(TEST_ARRAY);
        let sorted_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let merge_sort_result = merge_sort(&mut unsorted_vec);

        assert_eq!(*merge_sort_result, sorted_vec);

        let mut quick_sort_sortable = unsorted_vec.to_vec();

        quick_sort(&mut quick_sort_sortable);
        assert_eq!(quick_sort_sortable, sorted_vec);

        let bubble_sort_result = bubble_sort(&mut unsorted_vec);
        assert_eq!(*bubble_sort_result, sorted_vec);

        let insertion_sort_result = insertion_sort(&unsorted_vec);
        assert_eq!(insertion_sort_result, sorted_vec);

        let mut selection_sort_sortable = unsorted_vec.to_vec();

        selection_sort(&mut selection_sort_sortable);
        assert_eq!(selection_sort_sortable, sorted_vec);
    }
}
