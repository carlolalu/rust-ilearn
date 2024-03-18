// Problems to address:

//      1. First, our main function now performs two tasks: it parses arguments and reads files. It’s best to separate functionality so each function is responsible for one task.

//      2. This issue also ties into the second problem: although query and file_path are configuration variables to our program, variables like contents are used to perform the program’s logic. It’s best to group the configuration variables into one structure to make their purpose clear.

//      3. The third problem is that we’ve used expect to print an error message when reading the file fails, but the error message just prints Should have been able to read the file. Right now, regardless of the situation, we’d print the same error message for everything, which wouldn’t give the user any information!

//      4. Fourth, we use expect repeatedly to handle different errors, and if the user runs our program without specifying enough arguments, they’ll get an index out of bounds error from Rust that doesn’t clearly explain the problem. Having all the error-handling code in one place will also ensure that we’re printing messages that will be meaningful to our end users.

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!(
        "the query you gave is {}\nthe file_path you gace is {}\n\n",
        config.query, config.file_path
    );

    let contents = fs::read_to_string(config.file_path).expect("Could not read the given filepath");

    println!(
        "The given file_path points ot a file with contents {}",
        contents
    );
}

struct Config {
    query: String,
    file_path: String,
}

// So now that the purpose of the parse_config function is to create a Config instance, we can change parse_config from a plain function to a function named new that is associated with the Config struct. Making this change will make the code more idiomatic.

// carlolalu: I guess this will also make the code easier to test.

impl Config {
    fn new(args: &[String]) -> Config {
        Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        }
    }
}
