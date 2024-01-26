use rand::Rng;
use std::{collections::HashMap, io};

fn main() {
    println!("Hello, world! This is a median and mode program.
    Given a vector, you'll have to answer what its median and mode are.
    The size of the vector and the upper and lower bounds of its numbers can be randomized, or you can choose those parameters yourself (to a degree).\n");

    let user_choice = choose_or_no();
    let unsorted_vector = vector_creator(user_choice);
    let mut sorted_vector = unsorted_vector.clone(); // i hope this doesn't have some borrowing/ownership shenanigans
    sorted_vector.sort();
    let median_vector = median_in_vector(&sorted_vector); // i really hope this doesn't have ownership shenanigans
    let mode_vector = mode_in_vector(&sorted_vector);
    let tuple_answer = user_answers(&unsorted_vector);

    let median_answer = tuple_answer.0;
    let mode_answer = tuple_answer.1; // idk why i have to do this
    if median_answer == median_vector && mode_answer == mode_vector {
        println!("Congrats, you got it correct!");
    } else {
        println!("Wrong! Try again! The median was {} and the mode was {}. If you're still confused, here's the vectorwhen sorted: {:?}", median_vector, mode_vector, 
        &sorted_vector);
    }
}

// user chooses whether or not to choose the vector's parameters
fn choose_or_no() -> bool {
    loop {
        println!("Do you want to choose the vector's parameters? Please answer y/yes or n/no.
        If you see this message more than once, that means your input was invalid, and if so, please try again.");

        let answer = user_input_str();

        match answer.as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => continue,
        }
    }
}

// io fn that specifically returns Strings
fn user_input_str() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    input = input.trim().to_lowercase();

    input
}

// function that determines whether to construct fully or partially random vector
fn vector_creator(choice: bool) -> Vec<i32> {
    if choice == false {
        let random_vector = fully_random_vector();
        random_vector
    } else {
        let tuple = choose_parameters();
        let chosen_vector = partial_random_vector(tuple);
        chosen_vector
    }
}

// io fn that specifically returns signed 32-bit integers
fn user_input_int() -> i32 {
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
        let mut size = user_input_str();
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
        let upper = user_input_int();
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
        let lower = user_input_int(); 
        if lower > -101 && lower < tup.1 {
            tup.2 = lower;
            break;
        } else {
            continue;
        }
    }

    tup
}

// given a sorted vector, find its median
fn median_in_vector(vector: &Vec<i32>) -> i32 {
    let len = vector.len();
    if len % 2 != 0 {
        let number = vector.get((len + 1) / 2).unwrap();
        *number
    } else {
        let number = (vector.get(len / 2).unwrap() + vector.get((len + 1) / 2).unwrap()) / 2;
        number
    }
}

// given a sorted vector, find its mode
// problem: this works but can't handle more than one answer. the user can be technically correct if there's more than answer
// problem: if there's no mode, there is no answer, but this one always returns an answer. if there's no mode, the answer returned will be the first item in vector
fn mode_in_vector(vector: &Vec<i32>) -> i32 {
    let mut tracker: HashMap<i32, u32> = HashMap::new();

    // initializes the values in the hashmap
    for i in vector {
        *tracker.entry(*i).or_insert(1) += 1;
    }

    // keeps track of the highest count
    let mut highest = *vector.get(0).unwrap();
    for (key, value) in &tracker {
        if *value > *tracker.get(&highest).unwrap() {
            highest = *key;
        }
    }
    
    highest
}

// prompts user to give the median and mode of an unsorted vector. has to receive an unsorted vector
// problem: like mode_in_vector(), this isn't correct if there's more than one mode in the vector or if there's no mode at all
fn user_answers(unsorted_vector: &Vec<i32>) -> (i32, i32) {
    println!("Now that a vector has been created, it's time to for a quiz: what is its median and mode?
    To answer, please type in the following format: 
    median 
    mode
    In essence, two numbers separated by pressing enter. 
    Another example: 
    90 
    5
    In that example, 90 was the median and 5 was the mode.
    No other format will be accepted.\n");

    println!("What is the median and mode of {:?}", unsorted_vector);
    let mut tuple = (1, 1,);
    // not very good cuz if there's an invalid input, program just crashes. fuck it we ball
    tuple.0 = user_input_int();
    tuple.1 = user_input_int();

    tuple
}

/* planning:
1. randomize size of vector, the upper and lower bounds for the numbers, and the numbers to be stored in the vector
2. either ask user for the size and upper/lower bounds, or randomize them
3. sort by lowest to highest and get the median. if i'm storing one by one, why not just pre-sort the numbers?
4. keep track of the mode via hashmap
5. print the vector to the console. ask user to give its mode and median
6. print to console the results and determine + tell user if they were correct

7. should i separate functionalities in multiple files?
functionalities:
a. first ask user if they want to choose
b. an api that takes an argument (true/false if choose) and returns a vector unsorted
c. a function that sorts the vector from lowest to high
d. given a sorted function, find the median
e. another api that tracks the mode
f. print vector in b. to console. ask user to find the median and mode
g. check the user's answer with d. and e., print d. and e. and tell user if they were right

8. i can combine c and d together
9. i can but i shouldn't. need to separate responsibilities

10. my brain is just about breaking from this and i'm only halfway done, not to mention the possible debugging and the frikking modularity i'd have to do
(separating functionality into files or modules)
*/
