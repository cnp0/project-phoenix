fn print_data_added_to_heap(data: &char) {
    println!("Added {} to the heap", data);
}

fn fn0() {
    let data = Box::new('a');

    print_data_added_to_heap(&data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap() {
        fn0();
    }
}
