fn inception_to_zero(x: i32) -> i32 {
    if x <= 0 {
        return x;
    }

    return inception_to_zero(x - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursion() {
        let x = 20;

        let result = inception_to_zero(x);
        assert_eq!(result, 0);
    }
}
