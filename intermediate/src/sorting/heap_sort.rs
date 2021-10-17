// TODO: write to use vectors (didn't debug)
fn move_down<T: Ord>(arr: &mut [T], mut root_i: usize) {
    let last_i = arr.len() - 1;
    loop {
        let i = 2 * root_i + 1;
        if i > last_i {
            break;
        }
        let j = i + 1;
        let max_i = if j <= last_i && arr[j] > arr[i] { j } else { i };

        if arr[max_i] > arr[root_i] {
            arr.swap(root_i, max_i);
        }
        root_i = max_i;
    }
}

fn heapify<T: Ord>(arr: &mut [T]) {
    let last_parent = (arr.len() - 2) / 2;
    for i in (0..=last_parent).rev() {
        move_down(arr, i);
    }
}

pub fn heap_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    heapify(arr);

    for end in (1..arr.len()).rev() {
        arr.swap(0, end);
        move_down(&mut arr[..end], 0);
    }
}
