// In this section, we’ll add the searching logic to the minigrep program using the test-driven development (TDD) process with the following steps:

//     Write a test that fails and run it to make sure it fails for the reason you expect.
//     Write or modify just enough code to make the new test pass.
//     Refactor the code you just added or changed and make sure the tests continue to pass.
//     Repeat from step 1!

// Though it’s just one of many ways to write software, TDD can help drive code design. Writing the test before you write the code that makes the test pass helps to maintain high test coverage throughout the process.

// We’ll test drive the implementation of the functionality that will actually do the searching for the query string in the file contents and produce a list of lines that match the query. We’ll add this functionality in a function called search.


// carlolalu: this TDD seems to me a good way to establish precise goals before proceeding.

use std::env;
use std::process;

use::minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "the query you gave is {}\nthe file_path you gace is {}\n\n",
        config.query, config.file_path
    );

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
