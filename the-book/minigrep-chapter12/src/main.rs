use std::env;
use std::fs;

fn main() {
    let args : Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("the query you gave is {}\nthe file_path you gace is {}\n\n", query, file_path);

    let contents = fs::read_to_string(file_path).expect("Could not read the given filepath");

    println!("The given file_path points ot a file with contents {}", contents);
}
