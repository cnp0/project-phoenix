// examples of basic operators
fn operators() {
    let and_op = true && true;
    assert_eq!(and_op, true);

    let or_op = true || false;
    assert_eq!(or_op, true);

    let bang_op = !true;
    assert_eq!(bang_op, false);

    // from rust by example (bitwise)
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operators() {
        operators();
    }
}
