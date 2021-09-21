// references
const EXAMPLE_INT: u32 = 300;
const EXAMPLE_REF: &u32 = &EXAMPLE_INT;

fn add_to_ref(value: &u32, add_value: u32) -> u32 {
    let new_value = *value + add_value;

    return new_value;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_references() {
        // dereferencing should evaluate to true
        assert_eq!(EXAMPLE_INT, *EXAMPLE_REF);

        // using functions to pass references to data around
        let some_value = 200;
        let some_other_value = 500;
        let new_value = add_to_ref(&some_value, some_other_value);

        assert_eq!(some_value + some_other_value, new_value);

        // allowed to alias vairables if the ref is immmutable
        let a = 'a';
        let ref_a1 = &a;
        let ref_a2 = &a;

        assert_eq!(ref_a1, ref_a2);
    }
}
