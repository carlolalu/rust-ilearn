use crate::List::{Cons, Nil};


enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}


fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(List::Nil))));
    println!("Hello, world!");
}
