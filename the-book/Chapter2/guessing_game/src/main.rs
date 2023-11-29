use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, user!");
    println!("This program will challenge you to guess a specific number between 1 and 100");

    // generation of the secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        // user input
        println!("Gimme an input: ");

        let mut user_try = String::new();
        io::stdin()
            .read_line(&mut user_try)
            .expect("Failed to read line!");

        let user_try: u32 = match user_try.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You stated {}", user_try);
        match user_try.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }


}
