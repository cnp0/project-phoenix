use std::cmp;

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
