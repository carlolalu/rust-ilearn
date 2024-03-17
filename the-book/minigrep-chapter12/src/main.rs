// Problems to address:

//      1. First, our main function now performs two tasks: it parses arguments and reads files. It’s best to separate functionality so each function is responsible for one task.

//      2. This issue also ties into the second problem: although query and file_path are configuration variables to our program, variables like contents are used to perform the program’s logic. It’s best to group the configuration variables into one structure to make their purpose clear.

//      3. The third problem is that we’ve used expect to print an error message when reading the file fails, but the error message just prints Should have been able to read the file. Right now, regardless of the situation, we’d print the same error message for everything, which wouldn’t give the user any information!

//      4. Fourth, we use expect repeatedly to handle different errors, and if the user runs our program without specifying enough arguments, they’ll get an index out of bounds error from Rust that doesn’t clearly explain the problem. Having all the error-handling code in one place will also ensure that we’re printing messages that will be meaningful to our end users.

use std::env;
use std::fs;

fn main() {
    let args : Vec<String> = env::args().collect();

    let config = parse_args(&args);

    println!("the query you gave is {}\nthe file_path you gace is {}\n\n", config.query, config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Could not read the given filepath");

    println!("The given file_path points ot a file with contents {}", contents);
}


// We can take another small step to improve the parse_config function further. At the moment, we’re returning a tuple, but then we immediately break that tuple into individual parts again. This is a sign that perhaps we don’t have the right abstraction yet. Another indicator that shows there’s room for improvement is the config part of parse_config, which implies that the two values we return are related and are both part of one configuration value. We’re not currently conveying this meaning in the structure of the data other than by grouping the two values into a tuple.

struct Config {
    query: String,
    file_path: String,
}


// The parse_config function then holds the logic that determines which argument goes in which variable and passes the values back to main. We still create the query and file_path variables in main, but main no longer has the responsibility of determining how the command line arguments and variables correspond.

// This rework may seem like overkill for our small program, but we’re refactoring in small, incremental steps. After making this change, run the program again to verify that the argument parsing still works. It’s good to check your progress often, to help identify the cause of problems when they occur.

fn parse_args(args : &[String]) -> Config {

    // We decide to clone for simplicity, but it could be implemented also with refs. In Chapter 13 there is a more efficient method to learn.
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config {query, file_path}
}