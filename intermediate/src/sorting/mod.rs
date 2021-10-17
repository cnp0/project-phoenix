// Sorting implementations
//   1. Without resources, attempt to implement
//   2. When stuck syntactically, disect and follow
//      https://github.com/TheAlgorithms/Rust/blob/master/src/sorting
//   3. Review study-program material
//   4. TODO: implement immutable versions
mod bubble_sort;
mod heap_sort;
mod insertion_sort;
mod merge_sort;
mod quick_sort;
mod radix_sort;
mod selection_sort;

#[cfg(test)]
mod tests {
    use super::*;
    use bubble_sort::bubble_sort;
    use heap_sort::heap_sort;
    use insertion_sort::insertion_sort;
    use merge_sort::merge_sort;
    use quick_sort::quick_sort;
    use radix_sort::radix_sort;
    use selection_sort::selection_sort;

    const TEST_ARRAY: [u8; 10] = [10, 1, 2, 3, 4, 6, 5, 9, 8, 7];

    #[test]
    fn test_sorting() {
        let mut unsorted_vec = Vec::from(TEST_ARRAY);
        let sorted_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let merge_sort_result = merge_sort(&mut unsorted_vec);
        assert_eq!(*merge_sort_result, sorted_vec);

        let mut quick_sort_sortable = unsorted_vec.clone();
        quick_sort(&mut quick_sort_sortable);
        assert_eq!(quick_sort_sortable, sorted_vec);

        let bubble_sort_result = bubble_sort(&mut unsorted_vec);
        assert_eq!(*bubble_sort_result, sorted_vec);

        let insertion_sort_result = insertion_sort(&unsorted_vec);
        assert_eq!(insertion_sort_result, sorted_vec);

        let mut selection_sort_sortable = unsorted_vec.clone();
        selection_sort(&mut selection_sort_sortable);
        assert_eq!(selection_sort_sortable, sorted_vec);

        let mut radix_sort_sortable = unsorted_vec.clone();
        radix_sort(&mut radix_sort_sortable);
        assert_eq!(radix_sort_sortable, sorted_vec);

        let mut heap_sort_sortable = unsorted_vec.clone();
        heap_sort(&mut heap_sort_sortable);
        assert_eq!(heap_sort_sortable, sorted_vec);
    }
}
