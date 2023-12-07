// program to calculate pi and euler with their respective taylor series

use std::io;

enum MathConstant {
    pi,
    e,
    none,
}


fn main() {

    let choices : (MathConstant, u32) = (MathConstant::none, 0);

    loop {
        println!("Do you wanto to calculate '\u{CF80}' or 'e'? (p/e) ?");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: char = match choice.trim().parse() {
            Ok(single_char) => single_char,
            Err(_) => {
                println!("Please input only a single charachter between 'p' and 'e'");
                continue; 
            }
        };


        let choices.0 = match {
            'p' => MathConstant::pi,
            'e' => MathConstant::e,
            _ => {
                println!("Please input only a single charachter between 'p' and 'e'");
                continue;
            }
        }

        break;
    }

    loop {
        println!("At which (n-th) digit do you want to stop?");

        let mut last_digit = String::new();
        io::stdin()
            .read_line(&mut last_digit)
            .expect("Failed to read line");

        choices.1 = match last_digit.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break;
    }

    let constant = calculate(choices.0, choices.1);
    println!("The constant you wanted calculated till the {} digit is {}", choices.1, constant);
}





fn calculate(mathConst : MathConstant, digit : u32) -> f64 {
    let constant = match mathConst {
        pi => calculate_pi(digit),
        e => calculate_e(digit),
        none => 0.0,
    };
    constant
}

fn calculate_pi(last_digit : u32) -> f64 {
    // algo to calculate pi with the arctan
    4.0
}

fn calculate_e(last_digit : u32) -> f64 {
    // algo to calculate e with the taylor expansion of e^x
    4.0
}

