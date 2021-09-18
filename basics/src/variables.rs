#[allow(dead_code)]
fn variables() {
    // example variables
    let mut mutable_string = "something".to_owned();
    mutable_string.push_str(&" else");

    let mut mutable_int: i32 = 0;
    mutable_int += 1;

    println!("Mutable int: {}", mutable_int);

    let mut mutable_float: f64 = 0.;
    mutable_float -= 1.;

    println!("Mutable float: {}", mutable_float);

    let mut mutable_array = [0, 0, 0];
    mutable_array[0] = 1;
 
    let mut mutable_bool = true;
    println!("Mutable boolean: {}", mutable_bool);

    mutable_bool = false;
    println!("Mutable boolean: {}", mutable_bool);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variables() {
        variables();
    }
}