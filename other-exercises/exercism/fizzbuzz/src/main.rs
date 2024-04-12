// For each of them, if it's divisible by three, substitute the word "fizz"; if divisible by five, substitute "buzz"; if both, say both; if neither, say the number.

// (1) implement the fizzbuzz
// (2) implement a generalised version of fizzbuzz and write tests for it
// (3) if you can generalise again to the point in which you are ableto use traits and generics

use std::collections::HashMap;

fn main() {
    let (final_num, special_nums2words, special_nums) = get_constants();

    for num in 1..final_num + 1 {
        let output = elaborate_output(num, &special_nums2words, &special_nums);
        println!("{output}");
    }
}

fn get_constants<'a>() -> (i32, HashMap<i32, &'a str>, Vec<i32>) {
    let final_num: i32 = 100;

    let special_nums2words: HashMap<i32, &'static str> = HashMap::from([
        (3, "drei"),
        (5, "cinque"),
        (7, "sedem"),
        (9, "ennea"),
        (17, "sfiga"),
    ]);

    let mut special_nums: Vec<i32> = special_nums2words.keys().copied().collect();
    special_nums.sort();

    (final_num, special_nums2words, special_nums)
}

fn elaborate_output(
    num: i32,
    special_nums2words: &HashMap<i32, &str>,
    special_nums: &Vec<i32>,
) -> String {
    let mut output = String::new();

    for special_num in special_nums {
        if num % (*special_num) == 0 {
            output.push_str(special_nums2words.get(special_num).unwrap());
        }
    }

    if output.is_empty() {
        output.push_str(&num.to_string());
    }

    output
}

#[cfg(test)]
mod test {

    #[test]
    fn check_output_elaboration() {
        use crate::*;

        let (_final_num, special_nums2words, special_nums) = get_constants();

        let actual = elaborate_output(2, &special_nums2words, &special_nums);
        let expected = 2.to_string();
        assert_eq!(expected, actual);

        let actual = elaborate_output(27, &special_nums2words, &special_nums);
        let expected = "dreiennea".to_string();
        assert_eq!(expected, actual);

        let actual = elaborate_output(15, &special_nums2words, &special_nums);
        let expected = "dreicinque".to_string();
        assert_eq!(expected, actual);

        let actual = elaborate_output(45, &special_nums2words, &special_nums);
        let expected = "dreicinqueennea".to_string();
        assert_eq!(expected, actual);

        let actual = elaborate_output(105, &special_nums2words, &special_nums);
        let expected = "dreicinquesedem".to_string();
        assert_eq!(expected, actual);

        let actual = elaborate_output(51, &special_nums2words, &special_nums);
        let expected = "dreisfiga".to_string();
        assert_eq!(expected, actual);
    }
}
