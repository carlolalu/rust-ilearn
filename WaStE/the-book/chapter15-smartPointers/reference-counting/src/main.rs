// let us try to emulate the scenario that the book describes

use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}


fn main() {
    let list1 = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil)))));

    let a = Cons(0, Rc::clone(&list1));
    let b = Cons(100, Rc::clone(&list1));

    println!("Hello, world!");
}
