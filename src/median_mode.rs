use std::collections::HashMap;

// given a sorted vector, find its median
pub fn median_in_vector(vector: &Vec<i32>) -> f64 {
    let len = vector.len();
    if len % 2 != 0 {
        let number = *vector.get(((len + 1) / 2) - 1).unwrap();
        number.into()
    } else {
        let number: f64 = (vector.get(len / 2).unwrap() + vector.get(((len + 1) / 2) - 1).unwrap()) as f64 / 2.0;
        number
    }
}

// given a sorted vector, find its mode
// problem: this works but can't handle more than one answer. the user can be technically correct if there's more than answer
// problem: if there's no mode, there is no answer, but this one always returns an answer. if there's no mode, the answer returned will be the first item in vector
pub fn mode_in_vector(vector: &Vec<i32>) -> Vec<i32> {
    let mut tracker: HashMap<i32, u32> = HashMap::new();

    // initializes the values in the hashmap to keep track of mode(s)
    for i in vector {
        match tracker.get(i) {
            Some(count) => { tracker.insert(*i, count + 1); },
            None => { tracker.insert(*i, 1); },
        }
    }

    // keeps track of the highest count
    let mut counter = 0;
    for (_key, value) in &tracker {
        if *value > counter {
            counter = *value;
        }
    }

    // if the highest count is 1, there is no mode
    if counter == 1 {
        // let entire: Vec<i32> = vector.clone(); // funny thing: when this clones, i think it rearranges all the numbers to be in order lol. keeping it here 4 posterity
        return vec![]
    }

    // note: i think there's a debate on whether or not a set with no repeating numbers has a mode at all. in this one, i chose the simpler option and returned an empty vec
    
    let mut vec_of_modes: Vec<i32> = Vec::new();

    // check the modes
    for (key, value) in &tracker {
        if *value == counter  && !vec_of_modes.contains(key) {
            vec_of_modes.push(*key);
        }
    }

    vec_of_modes
}