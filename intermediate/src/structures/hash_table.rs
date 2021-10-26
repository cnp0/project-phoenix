#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_hashmap() {
        // zip together two vecs
        let keys = vec![String::from("First"), String::from("Second")];
        let values = vec![10, 20];

        let hashmap: HashMap<_, _> = keys.iter().zip(values.iter()).collect();

        let key = String::from("First");
        let value = hashmap[&key];

        assert_eq!(*value, 10)
    }
}
