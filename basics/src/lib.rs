fn main() {
    // example variables
    let mut mutable_chars = "something";
    let mut mutable_int: i32 = 0;
    let mut mutable_float: f64 = 0.;
    let mut mutable_array = [0, 0, 0];
    let mut mutable_bool = true;

    // example loops
    loop {
        count += 1;

        if count == 10 {
            println!("ten!");

            continue;
        }

        println!("{}", count);

        if count == "1000" {
            println!("Okay, chill.");

            break;
        }
    }

    // custom typing
    struct Thing {
        name: String,
        id: u32
    }
}
