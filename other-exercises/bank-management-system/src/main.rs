// source: https://hackr.io/blog/cpp-projects

// Next up, we’ll dive into financial management with a simple Bank System that lets users create an account with a basic registration system, not to mention deposit and withdraw money.

// I’m also going to be setting you the challenge of using object-oriented programming to create classes and objects that represent bank accounts.

// It’s a big leap from individual functions to designing a system using objects, encapsulating data, and operations.

// goals: you are a user on the terminal, you should be able to access your account, create one, deposit money and withdraw money. And visualise how much money you have on the account.

// todo: future exercise: when you will see how to use public and private in rust implement them into this program

// todo: very distant exercise: add a gui with some library you wanna learn

use std::io;

#[derive(Debug)]
struct BankAccount {
    user_name : String,
    user_passwd : String,
    id : u32,
    amount : i32,
}

impl BankAccount {

    fn new(user_name : String, user_passwd : String, id : u32, amount : i32) -> BankAccount {
        BankAccount{user_name, user_passwd, id, amount}
    }

    fn deposit(&mut self, sum : i32) {
        self.amount += sum;
    }

    fn withdraw(&mut self, sum : i32) {
        self.amount -= sum;
    }

    fn get_amount(&self) -> String {
        self.amount.to_string()
    }
}


fn get_validated_command() -> char {

    let validated_command = loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Please print only one char between '1','2' and '3'");

        let input: char = match input.trim().parse() {
            Ok(letter) => letter,
            Err(_) => {
                println!("Please print only one char between '1','2' and '3'");
                continue;
            }
        };

        // now we must validate the command
        if input!='1' && input!='2' && input !='3' {
            continue;
        }
        break input;
    };

    validated_command
}

fn get_input_string() -> String {
    
    let input = loop {

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input: String = match input.trim().parse() {
            Ok(line) => line,
            Err(_) => {
                println!("Failed to read line");
                continue;
            }
        };
        break input;
    };
    input
}

// todo: cure the UI

fn main() {

    println!("Welcome to carlolalu's bank!\nWe are a very tricky service, we let you open all the bank accounts you want...\n\nBut we never let you erase them, keeping all your data muahahahah\nWhat do you want to do?");

    loop {
        println!("Open a new account (1), Access my account (2), Quit (3) ?");

        let command = get_validated_command();

        if command=='1' {

            println!("Please input your username");
            let username = get_input_string();
            println!("Please input your password");
            let password = get_input_string();
            
            println!("Please deposit your money: how much of it do you have?");

            let amount = loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read num");
        
                let input: i32 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Failed to read num");
                        continue;
                    }
                };
                break input;
            };  

            println!("{:?}", BankAccount::new(username, password, 1, amount));


        } else if command=='2' {
            println!("I am sorry, the requested service has not been implemented yet!");
        } else {
            return;
        }
    }
}
