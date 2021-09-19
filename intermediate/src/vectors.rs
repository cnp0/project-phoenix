// vecs in rust are more akin to dynamic array data structures
// than standard rust arrays
fn create_vec() -> Vec<i32> {
    let vec = Vec::new();

    return vec;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vectors() {
        let mut example_vec = create_vec();

        example_vec.push(1);
        example_vec.push(2);

        assert!(example_vec.len() == 2);

        let indexed_element = example_vec[1];
        let popped_element = example_vec.pop();

        assert_eq!(Some(indexed_element), popped_element);
    }
}
