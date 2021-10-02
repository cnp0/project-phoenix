// to do something with the data
fn print_data_added_to_stack(data: char) {
    println!("Added {} to stack", data);
}

fn fn0() {
    let data = 'a';

    print_data_added_to_stack(data);
}

fn fn1() {
    let data = 'b';

    print_data_added_to_stack(data);
}

fn fn2() {
    let data = 'c';

    print_data_added_to_stack(data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        // data is stored to the stack in execution order
        // ---- psuedo-stack ----
        // fn2 contents
        // fn1 contents
        // fn0 contents
        // ----------------------

        // first-to-stack
        fn0();

        // second-to-stack
        fn1();

        // third-to-stack
        fn2();
    }
}
