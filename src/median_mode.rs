use std::collections::HashMap;

// given a sorted vector, find its median
pub fn median_in_vector(vector: &Vec<i32>) -> i32 {
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
pub fn mode_in_vector(vector: &Vec<i32>) -> i32 {
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