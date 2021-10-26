#[cfg(test)]
mod tests {
    #[test]
    fn test_bitwise_operations() {
        let a = 1; // 01
        let b = 2; // 10

        // 01 & 10 = 00
        assert_eq!(a & b, 0);

        // 01 | 10 = 11
        assert_eq!(a | b, 3);

        // 01 != 10 = 11
        assert_eq!(a ^ b, 3);

        // 01 << 10 = 100
        assert_eq!(a << b, 4);

        // 01 >> 10 = 0
        assert_eq!(a >> b, 0);

        // NOT 01 = -2
        assert_eq!(!a, -2);

        // unsigned 8-bit is 11111111 - 01 (255 - 1)
        assert_eq!(!1 as u8, 254);
    }
}
