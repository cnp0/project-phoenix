fn main() {
    // example variables
    let mut mutable_chars = "something".to_owned();
    mutable_chars.push_str(&" else");

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

    // infinite loop
    let mut count = 0;
    
    loop {
        count += 1;

        if count == 10 {
            println!("ten!");

            continue;
        }

        if count == 1000 {
            println!("Okay, chill.");

            break;
        }
    }

    // finite loop
    for n in 1..1000 {
        if n == 999 {
            println!("One more loop!");
        }
    }

    // iterator (and matching)
    let things = vec!["first", "second", "third"];

    for thing in things.into_iter() {
        match thing {
            "second" => println!("Printing the second thing."),
            _ => println!("Printing something."),
        }
    }

    // custom typing
    struct Thing {
        name: String,
        id: u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}