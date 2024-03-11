
// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

// I will do what indicated but all commands will be in minuscule letters.

use std::collections::HashMap;
use std::io;

type Employee = String;
type Department = String;

#[derive(Debug,PartialEq)]
enum Cmd {
    Add((Employee,Department)),
    ListOnlyDepartment(Department),
    List,
    Quit,
}

#[derive(Debug)]
struct Company {
    name : String,
    h_resources : HashMap::<Department, Vec<Employee> >
    //departments : Vec<Department>,
    //employees : HashMap::<&'a Department, Vec<Employee> >,
}

impl Company {
    fn new(name : &str) -> Company {
        let company = Company{name: name.trim().to_string(), h_resources : HashMap::new() };
        company
    }
}


fn main() {

    let mut input = String::new();
    println!("Hello, enterpreneur, I am your new digital manager! Please tell me the name of the company I am now working for:");
    io::stdin().read_line(&mut input).expect("This company name could not be read properly, aufwiedersehen!");
    let mut our_company = Company::new(&input);

    println!("\nPerfect, our company is called {}, and has a strong potential.", our_company.name);

    loop {
        println!("\n######\nI am at your service, ready for a new command! Your options are: \n--------\n1. \"add <name> to <department-name>\"\n2. \"list [department-name]\"\n3. \"quit\"/\"q\"\n--------\n######\n");

        let cmd = accept_new_command();
        if let Cmd::Quit = cmd {
            break;
        }

        execute(cmd, &mut our_company);
    }

    println!("Till next time, goodbye!");
}




fn validate_n_elaborate(input : &str) -> Option<Cmd> {
    let words : Vec<&str> = input.split(" ").collect();
    let word0 = words.get(0)?;

    let cmd = match *word0 {
        "q" | "quit" => Some(Cmd::Quit),
        "list" => {
            match words.len() {
                1 => Some(Cmd::List),
                2 => Some(Cmd::ListOnlyDepartment(words[1].to_string())),
                _ => None,
            }
        }
        "add" => {
            if words.len() != 4 {
                return None;
            }

            match words[2] {
                "to" => Some(Cmd::Add((words[1].to_string(),words[3].to_string()))),
                _ => None,
            }
        }
        _ => None
    };
    cmd
}

fn accept_new_command() -> Cmd {
    let mut input = String::new();
    let cmd = loop {
        if let Err(err) = io::stdin().read_line(&mut input) {
            println!("{}", err);
            continue;
        }

        let input = input.trim();

        let cmd = validate_n_elaborate(input);
        match cmd {
            None => { println!(r#"The command "{}" was not understood!"#, input); continue; }
            Some(cmd) => break cmd,
        }
    };
    cmd
}

fn execute(cmd : Cmd, company : &mut Company) {

    match cmd {
        Cmd::Quit => return,
        Cmd::Add((employee, department)) => {
            if !company.h_resources.contains_key(&department) {
                company.h_resources.insert(department.clone(), Vec::new());
            }
            let dep = company.h_resources.get_mut(&department).unwrap();
            dep.push(employee);
        }
        Cmd::List => {
            //todo: in the future implement here the Display trait
            //todo: list in alphabetical order. Idea: the departments are a vector of strings, and then employees is an hashmap<&departments, employess>. Then I can use the sort methods of the vector and take the values out of the hashMap easily
            if company.h_resources.is_empty() {
                println!("We have no human resources at all! Are we some kinda shady offshore company?");
                return;
            }

            for dep in company.h_resources.keys() {
                println!("\n-----{}------\n", *dep);
                for employee in company.h_resources.get(dep).unwrap() {
                    println!("{}", employee);
                }
                println!("-----------\n")
            }
            println!("This are all our human resources");
        }
        Cmd::ListOnlyDepartment(dep_name) => {
            match company.h_resources.get(&dep_name) {
                None => println!("The requested department does not exist!"),
                Some(dep) => {
                    println!("\n-----{}------\n", dep_name);
                    for employee in dep {
                        println!("{}", employee);
                    }
                    println!("-----------\n")
                }
            }

        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_validation_add() {
        let validated = validate_n_elaborate("add sally to sales");
        assert_eq!(Some(Cmd::Add(("sally".to_string(),"sales".to_string()))),validated);
    }

    #[test]
    fn check_validation_list() {
        let validated = validate_n_elaborate("list");
        assert_eq!(Some(Cmd::List),validated);
        let validated = validate_n_elaborate("list sales");
        assert_eq!(Some(Cmd::ListOnlyDepartment("sales".to_string())),validated);
    }

    #[test]
    fn check_validation_quit() {
        let validated = validate_n_elaborate("q probabile");
        assert_eq!(Some(Cmd::Quit),validated);
        let validated = validate_n_elaborate("quit");
        assert_eq!(Some(Cmd::Quit),validated);
    }

    #[test]
    fn check_validation_none() {
        let validated = validate_n_elaborate("qu");
        assert_eq!(None,validated);
        let validated = validate_n_elaborate("parlea");
        assert_eq!(None,validated);
    }

    #[test]
    fn check_cmd_add() {
        let mut company = Company::new("Goodplace");
        execute(Cmd::Add(("sally".to_string(), "sales".to_string())), &mut company);

        let expected = vec!["sally".to_string()];

        assert_eq!(*company.h_resources.get("sales").unwrap(),expected);
    }

    #[test]
    #[should_panic(expected = "message")]
    fn another() {
        panic!("message");
    }
}