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
