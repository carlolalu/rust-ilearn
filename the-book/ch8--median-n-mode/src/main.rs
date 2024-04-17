use std::collections::HashMap;

struct MedianMode<T>(T, T, T);


fn main() {

    // future ideas (todo): generate a random vector with the rand library and thread spawns
    // use ::rand::Rng; rand = "0.9"

    let mut integers : Vec<i32> = vec![3, 4, 6, 6, 6, 5, 7, 9, 9, 28, 5, 6, 6, 6, 7, 8, 76];
    integers.sort();

    let median_n_mode = compute_median_n_mode(&integers);

    println!("The median of {:?} is {} and a mode is {} with {} occurences", integers, median_n_mode.0, median_n_mode.1, median_n_mode.2);

}


fn compute_median_n_mode(integers : &Vec<i32>) -> MedianMode::<i32> {

    let median = *integers.get(integers.len()/2).expect("the vector should not be empty");

    let mut num2occurrences : HashMap<i32, i32> = HashMap::new();
    for integer in integers {
        let count = num2occurrences.entry(*integer).or_insert(0);
        *count += 1;
    }

    let (mut mode, mut mode_occurrence) = (0,0);

    for (num, occurrence) in num2occurrences {
        if occurrence > mode_occurrence {
            (mode, mode_occurrence) = (num, occurrence);
        }
    }

    let median_n_mode = MedianMode::<i32>(median, mode, mode_occurrence);
    median_n_mode

}