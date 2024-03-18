
use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error> > {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }


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




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

fn search<'a>(query : &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::<&str>::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}