use std::io;

//DRY. io fn that specifically returns signed 32-bit integers
pub fn user_input_int() -> i32 {
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line!");

    let number: i32 = number
        .trim()
        .parse()
        .expect("You did not enter a number");

    number
}

// io fn that specifically returns Strings
pub fn user_input_str() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    input = input.trim().to_lowercase();

    input
}

