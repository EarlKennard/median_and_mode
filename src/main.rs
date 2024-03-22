pub mod user_inputs;
pub mod vector_creations;
pub mod median_mode;

use crate::user_inputs::{user_input_str, user_input_veci32, user_inputf64};
use crate::vector_creations::vector_creator;
use crate::median_mode::{median_in_vector, mode_in_vector};

fn main() {
    println!("Hello, world! This is a median and mode program.
    Given a vector, you'll have to answer what its median and mode are.
    The size of the vector and the upper and lower bounds of its numbers can be randomized, or you can choose those parameters yourself (to a degree).\n");

    let user_choice = choose_or_no();
    let unsorted_vector = vector_creator(user_choice);
    let mut sorted_vector = unsorted_vector.clone(); // i'd prefer not to use clone() but i don't think this is too bad
    sorted_vector.sort();
    let median_vector = median_in_vector(&sorted_vector);
    let mode_vector = mode_in_vector(&sorted_vector);

    let median_answer = median_answer(&unsorted_vector);
    median_checker(median_vector, median_answer);

    let mode_answer = mode_answer(&unsorted_vector);
    mode_checker(&mode_vector, mode_answer);
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

fn mode_answer(unsorted_vector: &Vec<i32>) -> Vec<i32> {
    println!("Here is the unsorted vector: {:?}", unsorted_vector);
    println!("What is the vector's mode? If there is more than one, please separate each mode by a space. If there is no mode, press enter for no answer.");

    let user_mode_vec = user_input_veci32();
    user_mode_vec
}

fn median_answer(unsorted_vector: &Vec<i32>) -> f64 {
    println!("Here is the unsorted vector: {:?}", unsorted_vector);
    println!("What is the vector's median? There can only be one answer.");

    let answer = user_inputf64();
    answer
}

fn mode_checker(answers: &Vec<i32>, input: Vec<i32>) {
    let correct_answers: Vec<_> = input.into_iter().filter(|item| answers.contains(item)).collect();

    if correct_answers.len() == answers.len() {
        println!("Congrats! You got all of the right answers.");
    } else if correct_answers.len() != 0 && correct_answers.len() < answers.len() {
        println!("Sorry! You only got some of the modes right. You get {} points.", correct_answers.len())
    } else {
        println!("Sorry! Your answer didn't contain a single correct mode.")
    }

    println!("Here is the set of correct modes: {:?}", answers);
}

fn median_checker(answer: f64, input: f64) {
    if answer == input {
        println!("Congrats! You got the median correct.");
    } else {
        println!("Oh no! You got it wrong. The correct median was {}", answer);
    }
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
a) i kind of get why i have to do pub mod [file] and then bring the relevant fns into scope by typing 'use', but fsr this 'pub mod user_inputs;'
also affects vector_creations.rs. they both have to use user_inputs.rs, so i have to bring its fns into scope with 'use ...' in both files, but idk why using the aforementioned
pub mod in main also affects the visibility of user_inputs in vector_creations. is it cuz main uses vector_creations, and so declaring it in main affects vector_creations? 
weird.
b) also, using 'pub mod user_inputs;' in vector_creations gives an error. it's like it expects a folder called vector_creations where user_inputs will be. either a folder
or a module that contains user_inputs. man this is so weird. 

i kind of get rust modules, but at the same time I really don't.
*/
