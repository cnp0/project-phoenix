pub fn radix_sort(vec: &mut Vec<u8>) {
    let max_ele = match vec.iter().max() {
        Some(&x) => x as usize,
        None => return,
    };

    let radix = vec.len().next_power_of_two();

    // start at LSD
    let mut place = 1;
    while place <= max_ele {
        let digit_of = |x| x as usize / place % radix;

        // count digit occurances
        let mut counter = vec![0; radix];
        for &x in vec.iter() {
            counter[digit_of(x)] += 1;
        }

        // compute last index of each digit
        for i in 1..radix {
            counter[i] += counter[i - 1];
        }

        // write elements to their new indices
        for &x in vec.to_owned().iter().rev() {
            counter[digit_of(x)] -= 1;
            vec[counter[digit_of(x)]] = x;
        }

        place *= radix;
    }
}
