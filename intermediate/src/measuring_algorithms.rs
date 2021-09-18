#[allow(dead_code)]
const ALPHABET_LEN: usize = 26;

#[allow(dead_code)]
const ALPHABET: [char; ALPHABET_LEN] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

#[allow(dead_code)]
struct SpecialAlphabet {
    letters: [char; ALPHABET_LEN],
    len: usize,
}

#[allow(dead_code)]
const SPECIAL_ALPHABET: SpecialAlphabet = SpecialAlphabet {
    letters: ALPHABET,
    len: ALPHABET_LEN,
};

#[allow(dead_code)]
fn size_of_array(special_array: SpecialAlphabet) -> usize {
    return special_array.len;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_special_alphabet() {
        let result: usize = size_of_array(SPECIAL_ALPHABET);
        assert_eq!(ALPHABET_LEN, result);
    }
}
