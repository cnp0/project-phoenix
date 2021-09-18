#[allow(dead_code)]
fn infinite_loop(target_num: u32) -> u32 {
    let mut count = 0;
    
    loop {
        count += 1;

        if count == target_num {
            return count;
        }
    }
}

#[allow(dead_code)]
fn finite_loop(target_num: u32) -> u32 {
    let mut count = 0;

    // _ prefixed variables when intentionally unused
    for _n in 1..=target_num {
        count += 1;
    }

    return count;
}

#[allow(dead_code)]
fn iterator(strings: Vec<String>, target_string: String) -> bool {
    for string in strings.into_iter() {
        if string == target_string {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infinite() {
        let target_num = 1000;

        let infinite_result: u32 = infinite_loop(target_num);
        assert_eq!(target_num, infinite_result);
    }

    #[test]
    fn test_finite() {
        let target_num = 2000;

        let finite_result: u32 = finite_loop(target_num);
        assert_eq!(target_num, finite_result);
    }

    #[test]
    fn test_iterator() {

        // where target is found
        let strings = vec![String::from("first"), String::from("second"), String::from("third")];
        let strings_clone = strings.clone();

        let successful_target = String::from("third");

        let successful_result: bool = iterator(strings, successful_target);
        assert_eq!(successful_result, true);

        let unsuccessful_target = String::from("I am missing");
        
        let unsuccessful_result: bool = iterator(strings_clone, unsuccessful_target);
        assert_eq!(unsuccessful_result, false);

        
    }
}