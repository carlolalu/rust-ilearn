// Problems to address:

//      4. Fourth, we use expect repeatedly to handle different errors, and if the user runs our program without specifying enough arguments, they’ll get an index out of bounds error from Rust that doesn’t clearly explain the problem. Having all the error-handling code in one place will also ensure that we’re printing messages that will be meaningful to our end users.

// Separation of Concerns for Binary Projects

// The organizational problem of allocating responsibility for multiple tasks to the main function is common to many binary projects. As a result, the Rust community has developed guidelines for splitting the separate concerns of a binary program when main starts getting large. This process has the following steps:

//     Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
//     As long as your command line parsing logic is small, it can remain in main.rs.
//     When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.

// The responsibilities that remain in the main function after this process should be limited to the following:

//     Calling the command line parsing logic with the argument values
//     Setting up any other configuration
//     Calling a run function in lib.rs
//     Handling the error if run returns an error

// This pattern is about separating concerns: main.rs handles running the program, and lib.rs handles all the logic of the task at hand. Because you can’t test the main function directly, this structure lets you test all of your program’s logic by moving it into functions in lib.rs. The code that remains in main.rs will be small enough to verify its correctness by reading it. Let’s rework our program by following this process.

use std::env;
use std::process;

// note, from chapter 7:

//    We mentioned a package can contain both a src/main.rs binary crate root as well as a src/lib.rs library crate root, and both crates will have the package name by default. Typically, packages with this pattern of containing both a library and a binary crate will have just enough code in the binary crate to start an executable that calls code with the library crate. This lets other projects benefit from the most functionality that the package provides, because the library crate’s code can be shared.

//    The module tree should be defined in src/lib.rs. Then, any public items can be used in the binary crate by starting paths with the name of the package. The binary crate becomes a user of the library crate just like a completely external crate would use the library crate: it can only use the public API. This helps you design a good API; not only are you the author, you’re also a client!

//    In Chapter 12, we’ll demonstrate this organizational practice with a command-line program that will contain both a binary crate and a library crate.

// This should explain why here we import with minigrep::

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

    // I guess we can see the function `run()` because it is a sibling of `Config`, and sibling are able to see in each others scope
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
