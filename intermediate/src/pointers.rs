// references
const EXAMPLE_INT: u32 = 300;
const EXAMPLE_REF: &u32 = &EXAMPLE_INT;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_references() {
        // dereferencing should evaluate to true
        assert_eq!(EXAMPLE_INT, *EXAMPLE_REF);
    }
}
