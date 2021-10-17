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
