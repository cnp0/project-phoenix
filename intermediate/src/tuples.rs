// unlike arrays, tuples have specific order and can hold
// multiple data types

const EXAMPLE_TUPLE: (f32, u8, &str) = (0.3, 5, &"Something");
const EXAMPLE_TUPLE_TUPLE: ((f32, u8, &str), (f32, u8, &str)) = (EXAMPLE_TUPLE, EXAMPLE_TUPLE);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuples() {
        // you can print these! NOTE: a macro is used to do this
        // and through this you can only print a tuple up to
        // 12 elements in size.
        println!("Just a tuple: {:?}", EXAMPLE_TUPLE);
        println!("A tuple of tuples: {:?}", EXAMPLE_TUPLE_TUPLE);

        // not so hot indexing
        assert_eq!(EXAMPLE_TUPLE_TUPLE.0 .0, EXAMPLE_TUPLE.0);

        // you can also do this!
        let (var1, var2, var3) = EXAMPLE_TUPLE;
        assert_eq!(var1, EXAMPLE_TUPLE.0);
        assert_eq!(var2, EXAMPLE_TUPLE.1);
        assert_eq!(var3, EXAMPLE_TUPLE.2);
    }
}
