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
