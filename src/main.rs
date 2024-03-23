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
    if answers.len() == 0 && input.len() > 0 {
        println!("Sorry! Your answer didn't contain a single correct mode.");
    }

    let correct_answers: Vec<_> = input.clone().into_iter().filter(|item| answers.contains(item)).collect();
    // i don't like this clone()

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
        println!("Oh no! You got it wrong. The correct median was {}.", answer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn median_checker_mirror(answer: f64, input: f64) -> String {
        // this is incredibly hacky and i don't like it, and it may even be unacceptable
        // i'm not returning &str cuz it requires a static lifetime and idk how i feel about that, i don't really like it

        if answer == input {
            String::from("Correct")
        } else {
            String::from("Wrong")
        }
    }

    #[test]
    fn median_checker_correct_answer() {
        assert_eq!(median_checker_mirror(3.0, 3.0), "Correct");
    }

    #[test]
    fn median_checker_wrong_answer() {
        assert_eq!(median_checker_mirror(3.0, 2.0), "Wrong");
    }

    fn mode_checker_mirror(answers: &Vec<i32>, input: Vec<i32>) -> String {
        // also incredibly hacky

        if answers.len() == 0 && input.len() > 0 {
            return String::from("Sorry! Your answer didn't contain a single correct mode.")
        }

        let correct_answers: Vec<_> = input.clone().into_iter().filter(|item| answers.contains(item)).collect();

        if correct_answers.len() == answers.len() {
            String::from("Congrats! You got all of the right answers.")
        } else if correct_answers.len() != 0 && correct_answers.len() < answers.len() {
            String::from("Sorry! You only got some of the modes right.")
        } else {
            String::from("Sorry! Your answer didn't contain a single correct mode.")
        }
    }

    #[test]
    fn mode_checker_all_right() {
        let answer_vector = vec![2, 3, 4, 5];
        let input_vector = vec![5, 4, 3, 2];

        assert_eq!(mode_checker_mirror(&answer_vector, input_vector), "Congrats! You got all of the right answers.");
    }

    #[test]
    fn mode_checker_all_right_empty() {
        let answer_vector = vec![];
        let input_vector = vec![];

        assert_eq!(mode_checker_mirror(&answer_vector, input_vector), "Congrats! You got all of the right answers.");
    }

    #[test]
    fn mode_checker_some_right_smaller_input() {
        let answer_vector = vec![2, 3, 4, 5];
        let input_vector = vec![5, 4, 3];

        assert_eq!(mode_checker_mirror(&answer_vector, input_vector), "Sorry! You only got some of the modes right.");
    }

    #[test]
    fn mode_checker_some_right_bigger_input() {
        let answer_vector = vec![2, 3, 4, 5];
        let input_vector = vec![5, 4, 3, 1, 6, 7];

        assert_eq!(mode_checker_mirror(&answer_vector, input_vector), "Sorry! You only got some of the modes right.");
    }

    #[test]
    fn mode_checker_none_right_empty_vec() {
        let answer_vector = vec![2, 3, 4, 5];
        let input_vector = vec![];

        assert_eq!(mode_checker_mirror(&answer_vector, input_vector), "Sorry! Your answer didn't contain a single correct mode.");
    }

    #[test]
    fn mode_checker_none_right_at_all() {
        let answer_vector = vec![2, 3, 4, 5];
        let input_vector = vec![6, 7, 8, 9, 10, 11];

        assert_eq!(mode_checker_mirror(&answer_vector, input_vector), "Sorry! Your answer didn't contain a single correct mode.");
    }

    #[test]
    fn mode_checker_correct_empty_vec() {
        let answer_vector = vec![];
        let input_vector = vec![6, 7, 8, 9, 10, 11];

        assert_eq!(mode_checker_mirror(&answer_vector, input_vector), "Sorry! Your answer didn't contain a single correct mode.");
    }
}
