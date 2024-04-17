// program to calculate pi and euler with their respective taylor series

use std::io;

enum MathConstant {
    Pi,
    E,
    none,
}


fn main() {

    let mut choices : (MathConstant, u32) = (MathConstant::none, 0);

    loop {
        println!("Do you wanto to calculate 'pi' or 'e'? (p/e) ?");

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


        choices.0 = match choice {
            'p' => MathConstant::Pi,
            'e' => MathConstant::E,
            _ => {
                println!("Please input only a single charachter between 'p' and 'e'");
                continue;
            }
        };
        break;
    }

    // mo stiamo a vedere

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





fn calculate(math_const : MathConstant, digit : u32) -> f64 {
    let constant = match math_const {
        MathConstant::Pi => calculate_pi(digit),
        MathConstant::E => calculate_e(digit),
        MathConstant::none => 0.0,
    };
    constant
}



fn calculate_pi(last_digit : u32) -> f64 {
    // algo to calculate pi with the arctan
    let mut pi_approximation : f64 = 0.0;
    //todo: calculate not till the N-th summand but till the N-th digit (future exercise)
    let last_summand = last_digit;
    for k in 0..last_summand {

        let sgn = if k%2==0 {
                1.0
            } else {
                -1.0
            };

        pi_approximation += sgn * 4.0 * ( 1.0/(2.0*(k as f64)+1.0 ) );
    }
    pi_approximation
}

fn calculate_e(last_digit : u32) -> f64 {
    // algo to calculate e with the taylor expansion of e^x
    let mut e_approximation : f64 = 0.0;
    //todo: calculate not till the N-th summand but till the N-th digit (future exercise)
    let last_summand = last_digit;

    for k in 0..last_summand {

        let factorial = |x : u32| -> u32 {
            let mut result : u32 = 1;
            for y in 1..=x {
                result *= y;
            }
            result
        };

        e_approximation += 1.0/(factorial(k) as f64)
    }

    e_approximation
}

