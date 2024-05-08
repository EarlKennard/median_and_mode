mod median_mode;
mod user_inputs;
mod vector_creations;

use crate::median_mode::{median_in_vector, mode_in_vector};
use crate::user_inputs::{user_input_str, user_input_veci32};
use crate::vector_creations::vector_creator;

fn main() {
    println!("Hello, world! This is a median and mode program.
    Given a vector, you'll have to answer what its median and mode are.
    The size of the vector and the upper and lower bounds of its numbers can be randomized, or you can choose those parameters yourself (to a degree).\n");

    let user_choice = choose_or_no();
    let unsorted_vector = vector_creator(user_choice);
    let mut sorted_vector = unsorted_vector.clone(); // i'd prefer not to use clone() but i don't think this is too bad
    sorted_vector.sort();
    let median_f64 = median_in_vector(&sorted_vector);
    let mut mode_vector = mode_in_vector(&sorted_vector);

    let median_user_answer = median_answer(&unsorted_vector);
    println!("{}", median_checker(median_f64, median_user_answer));
    println!("The correct median was {}.", median_f64);

    let mode_answer = mode_answer(&unsorted_vector);
    println!("{}", mode_checker(&mode_vector, mode_answer));
    mode_vector.sort();
    println!("Here is the set of correct modes: {:?}.", mode_vector);
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

// maybe a TODO: this one 'accepts' invalid inputs like words or letters and just skips over them,
// unlike the other user input related fns. i don't think it's bad, but it's good to remind myself
// that it exists
fn mode_answer(unsorted_vector: &[i32]) -> Vec<i32> {
    println!("Here is the unsorted vector: {:?}", unsorted_vector);
    println!("What is the vector's mode? If there is more than one, please separate each mode by a space. If there is no mode, press enter for no answer.");

    user_input_veci32()
}

fn median_answer(unsorted_vector: &[i32]) -> f64 {
    let outer: f64;

    loop {
        println!("Here is the unsorted vector: {:?}", unsorted_vector);
        println!("What is the vector's median? There can only be one answer. If you see this again, there was an invalid input.");

        let inner = user_input_str();
        let _inner: f64 = match inner.trim().parse() {
            Ok(num) => {
                outer = num;
                break;
            }
            Err(_) => continue,
        };
    }

    outer
}

fn mode_checker(answers: &[i32], input: Vec<i32>) -> &str {
    // there are only three major cases:
    // 1. correct answers is the same length as answers and input. so the user is completely right
    // 2. correct answers is empty, which means the user got none right. however, there's a chance
    //    that there is no mode and answers is empty, which means correct answers can be empty, so
    //    case 1 has to be caught first
    // 3. correct answers is not empty, but is not case 1, which means the user is only partially
    //    right. however way you slice it, as long as the following isn't true: { correct answers ==
    //    answers == input }, then the user is always partially right

    // filter the correct answers in input
    let correct_answers: Vec<_> = input.iter().filter(|item| answers.contains(item)).collect();

    if answers.len() == correct_answers.len() && answers.len() == input.len() {
        "Congrats, you got all the right modes."
        // case 1. there is a bug in this where if the user put in invalid answers like 'yes yes
        // yes', and the answers vec is empty, then the user is technically correct. i'll just let
        // that one go.
    } else if correct_answers.is_empty() {
        // case 2
        "Sorry, you got none of the modes right. See below for the full list of correct modes."
    } else {
        // case 3
        "Sorry, you only got some of the modes correct. See below for the full list of correct modes."
    }
}

fn median_checker(answer: f64, input: f64) -> String {
    if answer == input {
        "Congrats, you got the median correct.".to_string()
    } else {
        "Sorry, you got it wrong. See below for the correct median.".to_string()
    }
}

#[cfg(test)]
mod median_tests {
    use super::*;

    #[test]
    fn median_checker_correct_answer() {
        assert_eq!(
            median_checker(3.0, 3.0),
            "Congrats, you got the median correct.".to_string()
        );
    }

    #[test]
    fn median_checker_wrong_answer() {
        assert_eq!(
            median_checker(3.0, 2.0),
            "Sorry, you got it wrong. See below for the correct median.".to_string()
        );
    }
}

#[cfg(test)]
mod mode_tests {
    use super::*;

    #[test]
    fn mode_checker_all_right() {
        let answer_vector = vec![2, 3, 4, 5];
        let input_vector = vec![5, 4, 3, 2];

        assert_eq!(
            mode_checker(&answer_vector, input_vector),
            "Congrats, you got all the right modes."
        );
    }

    #[test]
    fn mode_checker_all_right_empty() {
        let answer_vector = vec![];
        let input_vector = vec![];

        assert_eq!(
            mode_checker(&answer_vector, input_vector),
            "Congrats, you got all the right modes."
        );
    }

    #[test]
    fn mode_checker_some_right_smaller_input() {
        let answer_vector = vec![2, 3, 4, 5];
        let input_vector = vec![5, 4, 3];

        assert_eq!(mode_checker(&answer_vector, input_vector), "Sorry, you only got some of the modes correct. See below for the full list of correct modes.");
    }

    #[test]
    fn mode_checker_some_right_bigger_input() {
        let answer_vector = vec![2, 3, 4, 5];
        let input_vector = vec![5, 4, 3, 1, 6, 7];

        assert_eq!(mode_checker(&answer_vector, input_vector), "Sorry, you only got some of the modes correct. See below for the full list of correct modes.");
    }

    #[test]
    fn mode_checker_none_right_empty_input() {
        let answer_vector = vec![2, 3, 4, 5];
        let input_vector = vec![];

        assert_eq!(
            mode_checker(&answer_vector, input_vector),
            "Sorry, you got none of the modes right. See below for the full list of correct modes."
        );
    }

    #[test]
    fn mode_checker_none_right_at_all() {
        let answer_vector = vec![2, 3, 4, 5];
        let input_vector = vec![6, 7, 8, 9, 10, 11];

        assert_eq!(
            mode_checker(&answer_vector, input_vector),
            "Sorry, you got none of the modes right. See below for the full list of correct modes."
        );
    }

    #[test]
    fn mode_checker_answer_empty_input_isnt() {
        let answer_vector = vec![];
        let input_vector = vec![6, 7, 8, 9, 10, 11];

        assert_eq!(
            mode_checker(&answer_vector, input_vector),
            "Sorry, you got none of the modes right. See below for the full list of correct modes."
        );
    }
}
