// purpose: input your monthly income and expense, and calculate how much you can spare in x months/years

use std::io;

fn main() {
    println!("Hello, world!");

    let monthly_income = receive_positive_input("Monthly income: ");
    let monthly_expenses = receive_positive_input("Monthly expenses: ");

    let months = receive_positive_input("Amount of time in months in which you want to spare: ");

    let total_spare_money = (monthly_income - monthly_expenses)*months;

    println!("The amount of wealth you can spare in {} months with a monthly expense of {} and a monthly income of {} is {}", months, monthly_expenses, monthly_income, total_spare_money);    
}


fn receive_positive_input(msg: &str) -> f64 {
    loop{
        println!("{msg}");

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line!");
        
        let parsed_input : f64 = match input.trim().parse() {
            Ok(num) => if num > 0.0 { num } else { continue; },
            Err(_) => continue,
        };
        return parsed_input;
    }
}