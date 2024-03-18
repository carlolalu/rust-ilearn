// Problems to address:

//      3. The third problem is that we’ve used expect to print an error message when reading the file fails, but the error message just prints Should have been able to read the file. Right now, regardless of the situation, we’d print the same error message for everything, which wouldn’t give the user any information!

//      4. Fourth, we use expect repeatedly to handle different errors, and if the user runs our program without specifying enough arguments, they’ll get an index out of bounds error from Rust that doesn’t clearly explain the problem. Having all the error-handling code in one place will also ensure that we’re printing messages that will be meaningful to our end users.

use std::env;
use std::fs;

// We’ve added a new use line to bring process from the standard library into scope. The code in the closure that will be run in the error case is only two lines: we print the err value and then call process::exit. The process::exit function will stop the program immediately and return the number that was passed as the exit status code. This is similar to the panic!-based handling we used in Listing 12-8, but we no longer get all the extra output.
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // To handle the error case and print a user-friendly message, we need to update main to handle the Result being returned by Config::build, as shown in Listing 12-10. We’ll also take the responsibility of exiting the command line tool with a nonzero error code away from panic! and instead implement it by hand. A nonzero exit status is a convention to signal to the process that called our program that the program exited with an error state.

    // In this listing, we’ve used a method we haven’t covered in detail yet: unwrap_or_else, which is defined on Result<T, E> by the standard library. Using unwrap_or_else allows us to define some custom, non-panic! error handling. If the Result is an Ok value, this method’s behavior is similar to unwrap: it returns the inner value Ok is wrapping. However, if the value is an Err value, this method calls the code in the closure, which is an anonymous function we define and pass as an argument to unwrap_or_else. We’ll cover closures in more detail in Chapter 13.

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

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

impl Config {
    // We can return a Result value that will contain a Config instance in the successful case and will describe the problem in the error case.
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Err("not enough arguments")
        } else {
            Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            })
        }
    }
}
