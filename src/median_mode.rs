use std::collections::HashMap;

// given a sorted vector, find its median
pub fn median_in_vector(vector: &[i32]) -> f64 {
    let len = vector.len();
    if len % 2 != 0 {
        let number = *vector.get(((len + 1) / 2) - 1).unwrap();
        number.into()
    } else {
        let number: f64 =
            (vector.get(len / 2).unwrap() + vector.get(((len + 1) / 2) - 1).unwrap()) as f64 / 2.0;
        number
    }
}

// given a sorted vector, find its mode
pub fn mode_in_vector(vector: &[i32]) -> Vec<i32> {
    let mut tracker: HashMap<i32, u32> = HashMap::new();

    // initializes the values in the hashmap to keep track of mode(s)
    for i in vector {
        match tracker.get(i) {
            Some(count) => {
                tracker.insert(*i, count + 1);
            }
            None => {
                tracker.insert(*i, 1);
            }
        }
    }

    // keeps track of the highest count
    let mut counter = 0;
    for value in tracker.values() {
        if *value > counter {
            counter = *value;
        }
    }

    // if the highest count is 1, there is no mode
    if counter == 1 {
        // let entire: Vec<i32> = vector.clone(); // funny thing: when this clones, i think it rearranges all the numbers to be in order lol. keeping it here 4 posterity
        return vec![];
    }

    // note: i think there's a debate on whether or not a set with no repeating numbers has a mode at all. in this one, i chose the simpler option and returned an empty vec

    let mut vec_of_modes: Vec<i32> = Vec::new();

    // check the modes
    for (key, value) in &tracker {
        if *value == counter && !vec_of_modes.contains(key) {
            vec_of_modes.push(*key);
        }
    }

    vec_of_modes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_median_odd_set() {
        let vector = vec![1, 2, 3, 4, 5];

        assert_eq!(median_in_vector(&vector), 3.0);
        assert_eq!(median_in_vector(&vector), 3 as f64);

        // note: rust does automatic conversion from i32/u32 to f64 when doing user inputs. but as far as I know, I can't test for assert_eq RHS w/ 3.0 as i32
        // maybe i can, but i'm just too lazy
    }

    #[test]
    fn vector_median_even_set() {
        let vector = vec![1, 2, 3, 4, 5, 6];

        assert_eq!(median_in_vector(&vector), 3.5);
    }

    #[test]
    fn mode_vector_no_mode() {
        let vector = vec![1, 2, 3, 4, 5];

        assert_eq!(mode_in_vector(&vector), vec![]);
    }

    #[test]
    fn mode_vector_one_mode() {
        let vector = vec![5, 6, 7, 10, 10, 20, 30];

        assert_eq!(mode_in_vector(&vector), vec![10]);
    }

    #[test]
    fn mode_vector_many_numbers_one_mode() {
        let vector = vec![1, 2, 3, 4, 4, 3, 5, 5, 3, 3, 6, 7, 3];

        assert_eq!(mode_in_vector(&vector), vec![3]);
    }

    #[test]
    fn mode_vector_six_different_modes() {
        let vector = vec![1, 2, 2, 3, 4, 4, 5, 5, 6, 6, 7, 8, 9, 10, 10, 11, 12, 11];
        let mut sorted = mode_in_vector(&vector);
        sorted.sort(); // i don't like how i have to do this separately

        assert_eq!(sorted, vec![2, 4, 5, 6, 10, 11]);
    }

    #[test]
    fn mode_vector_one_number_one_mode() {
        let vector = vec![1, 1, 1, 1, 1];

        assert_eq!(mode_in_vector(&vector), vec![1]);
    }
}
