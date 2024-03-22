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

pub fn user_inputf64() -> f64 {
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line!");

    let number: f64 = number
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


// this is for answering the mode(s)
pub fn user_input_veci32() -> Vec<i32> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let mut vec: Vec<i32> = Vec::new();

    for answer in input.split_whitespace() {
        if answer.parse::<i32>().is_ok() {
            vec.push(answer.parse::<i32>().unwrap());
        }
    }

    vec
}

