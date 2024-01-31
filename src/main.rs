pub mod user_inputs;
pub mod vector_creations;
pub mod median_mode;

use crate::user_inputs::user_input_str;
use crate::user_inputs::user_input_int;
use crate::vector_creations::vector_creator;
use crate::median_mode::median_in_vector;
use crate::median_mode::mode_in_vector;

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

11. i'm not too sure about modules
a) i kind of get why i have to do pub mod [file] and then bring the relevant fns into scope by typing 'use', but fsr this pub mod also affects vector_creations.rs.
they both have to use user_inputs.rs, so i have to bring its fns into scope with use, but idk why using pub mod in main affects the visibility of user_inputs 
in vector_creations. is it cuz main uses vector_creations, and so declaring it in main affects vector_creations? weird
b) also, using 'pub mod user_inputs;' in vector_creations gives an error. it's like it expects a folder called vector_creations where user_inputs will be. either a folder
or a module that contains user_inputs. man this is so weird. 

i kind of get rust modules, but at the same time I really don't.
*/
