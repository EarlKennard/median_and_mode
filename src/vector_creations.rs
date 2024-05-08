use crate::user_inputs::user_input_str;
use rand::Rng;

// function that determines whether to construct fully or partially random vector
pub fn vector_creator(choice: bool) -> Vec<i32> {
    match choice {
        false => {
            // this is the random vector
            fully_random_vector()
        }
        true => {
            // this is the user chosen vector
            let tuple = choose_parameters();
            partial_random_vector(tuple)
        }
    }
}

// fn that creates a fully randomized vector
fn fully_random_vector() -> Vec<i32> {
    let mut vector_size: u32 = rand::thread_rng().gen_range(5..=20);
    let upper_bound: i32 = rand::thread_rng().gen_range(-50..=100);
    let lower_bound: i32 = rand::thread_rng().gen_range(-100..upper_bound);

    let mut vector = Vec::new();
    while vector_size > 0 {
        let number = rand::thread_rng().gen_range(lower_bound..=upper_bound);
        vector.push(number);
        vector_size -= 1;
    }

    vector
}

// fn that creates a partially randomized vector, aka only the numbers are randomized
fn partial_random_vector((size, upper, lower): (u32, i32, i32)) -> Vec<i32> {
    let mut vector_partial_size = size;
    let mut vector = Vec::new();

    while vector_partial_size > 0 {
        let number = rand::thread_rng().gen_range(lower..=upper);
        vector.push(number);
        vector_partial_size -= 1;
    }

    vector
}

// fn that handles choosing parameters
fn choose_parameters() -> (u32, i32, i32) {
    println!("So you want to choose the parameters.");

    let mut tup: (u32, i32, i32) = (1, 1, 1);

    loop {
        println!("Choose the size of the vector. The minimum is 5 and the maximum is 20.
        If you see this message again, that means you chose a number outside of those limits or had an invalid input.");
        let size = user_input_str();
        let size: u32 = match size.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if size > 4 && size < 21 {
            tup.0 = size;
            break;
        } else {
            continue;
        }
    }

    loop {
        println!("Choose the upper bound of the vector. The minimum is -50 and the maximum is 100.
        If you see this message again, that means you chose a number outside of those limits or had an invalid input.");
        let upper = user_input_str();
        let upper: i32 = match upper.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if upper > -51 && upper < 101 {
            tup.1 = upper;
            break;
        } else {
            continue;
        }
    }

    loop {
        println!("Choose the lower bound of the vector. The minimum is -100 and the maximum is whatever you chose the upper bound to be.
        If you see this message again, that means you chose a number outside of those limits or had an invalid input.");
        let lower = user_input_str();
        let lower: i32 = match lower.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if lower > -101 && lower <= tup.1 {
            tup.2 = lower;
            break;
        } else {
            continue;
        }
    }

    tup
}
