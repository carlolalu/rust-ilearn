
use std::fs;
use std::error::Error;

// For the error type, we used the trait object Box<dyn Error> (and we’ve brought std::error::Error into scope with a use statement at the top). We’ll cover trait objects in Chapter 17. For now, just know that Box<dyn Error> means the function will return a type that implements the Error trait, but we don’t have to specify what particular type the return value will be. This gives us flexibility to return error values that may be of different types in different error cases.

pub fn run(config: Config) -> Result<(), Box<dyn Error> > {
    let contents = fs::read_to_string(config.file_path)?;

    println!(
        "The given file_path points ot a file with contents {}",
        contents
    );

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
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
