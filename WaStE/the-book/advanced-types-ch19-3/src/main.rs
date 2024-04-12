use std::io::Error;

type Result<T> = std::result::Result<T, std::io::Error>;

// the basic message of today is:

// the 'type =' syntax is better when we want to reduce visually the size of a type, while
// the newtype pattern (wrapping as this: 'struct Blabla(oldtype);') is better otherwise, because the type checker is ACTIVE and thus the compiler can help.

fn main() {
    println!("Hello, world!");
}
