// Mortgage Calculator - Calculate the monthly payments of a fixed term mortgage over given Nth terms at a given interest rate. Also figure out how long it will take the user to pay back the loan. For added complexity, add an option for users to select the compounding interval (Monthly, Weekly, Daily, Continually).

// terminology:

// principal = the amount of money lent to the borrower
// interest / interest_rate = the percentage of the loan to add to the debt, i.e. the cost of the loan itself
// term/terms = the amount of time required to pay the loan, usually given in years

// frm = fixed rate mortgage = it means the mortgage has a fixed interest rate
// arm = adjustable rm = the mortgage has a variable interest rate. calculations are here different and I ma sure here I could find interesting mathematical models

// idea of design. If I learn about struct I could define a struct Mortgage whcih calculates everything, also depending on the arm/frm typology

use std::io;

fn main() {
    println!("Hello, user!");
    println!("This program will calculate your monthly payment due to a mortgage with the parameters that you will provide");

    let (principal, terms_in_years, interest_in_percent) : (f64, f64, f64);

    principal = receive_input("Please input your loan: ");
    terms_in_years  = receive_input("Please input your term (in years): ");
    interest_in_percent  = receive_input("Please input your interest (as a float number from 1 to 100): ");

    let monthly_payment = calculate_monthly_payment(&principal, &terms_in_years, &interest_in_percent);

    println!("Your monthly payment is {}", monthly_payment);
}

fn receive_input(message: &str) -> f64 {
    loop {
        println!("{}",message);

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line!");

        let parsed_input : f64 = match user_input.trim().parse() {
            Ok(parsed_input) => parsed_input,
            Err(_) => continue,
        };
        return parsed_input;
    }
}

fn calculate_monthly_payment(principal : &f64, terms_in_years : &f64, interest_in_percent : &f64) -> f64 {
    let mut monthly_payment = principal/(terms_in_years *12.0);
    monthly_payment = monthly_payment * (1.0 + interest_in_percent/100.0);
    monthly_payment
}


